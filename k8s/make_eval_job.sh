#!/usr/bin/env bash
# Submit an eval Job for one or more runs sharing a base model.
#
#   k8s/make_eval_job.sh eval2-4b  Qwen/Qwen3-4B     "sft2-0 sft2-1 sft2-3"
#   k8s/make_eval_job.sh eval2-9b  Qwen/Qwen3.5-9B   "sft2-2 sft2-4"
#
# Optional 4th arg: checkpoints to score (default "final").
#   ... "sft2-0" "final checkpoint-41 checkpoint-82 checkpoint-123"
#
# SAMPLES=8 turns on best-of-k. Set OUT_TAG too, or the result overwrites the
# greedy one at the same path in the checkpoint repo:
#   SAMPLES=8 OUT_TAG=-bok8 k8s/make_eval_job.sh bok-4b Qwen/Qwen3-4B "sft2-0"
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
KUBECONFIG_FILE="${KUBECONFIG_FILE:-${KUBECONFIG:-}}"
KUBECTL="${KUBECTL:-kubectl}"
NS=default   # overridden by the cluster profile below

# Cluster profile — see the same block in make_jobs.sh. Defaults to
# research-common; boogiebonjour is kept only for reproducing old runs and is
# shared with another user, and turbox is retired (2026-08-24). The storage class
# is the difference that matters: weka-data is RWX, so an eval pod is not pinned
# to the node its PVC was created on.
CLUSTER="${CLUSTER:-research-common}"
case "$CLUSTER" in
  research-common)
    # Fallback when turbox is saturated by the arc-runners CI fleet, which
    # regularly holds 28 of its 32 GPUs. Namespace is assigned by the cluster
    # (jisenli2), not chosen -- the naming rule constrains OBJECT names, which
    # stay de2-rl-test-*. weka-data is a CSI-provisioned shared filesystem, so
    # the local-path node-pinning problem does not apply here either.
    : "${KUBECONFIG_FILE:=$HOME/.kube/research-common-h100}"
    NS="${NS_OVERRIDE:-jisenli2}"
    STORAGE_CLASS="${STORAGE_CLASS:-weka-data}"
    ACCESS_MODE="${ACCESS_MODE:-ReadWriteMany}"
    MEM_LIM="${MEM_LIM:-200Gi}"; MEM_REQ="${MEM_REQ:-64Gi}"
    BAD=() ;;
  turbox)
    # Retired 2026-08-24: turbox is not to be used. Its kubeconfig is archived at
    # ~/.kube/disabled/turbox-h100.yaml.disabled, so this branch would fail later
    # and less clearly anyway. Refuse up front instead.
    echo "CLUSTER=turbox is retired -- use CLUSTER=research-common" >&2; exit 1 ;;
  boogiebonjour)
    : "${KUBECONFIG_FILE:=$HOME/.kube/boogiebonjour}"
    STORAGE_CLASS="${STORAGE_CLASS:-local-path}"
    ACCESS_MODE="${ACCESS_MODE:-ReadWriteOnce}"
    MEM_LIM="${MEM_LIM:-400Gi}"; MEM_REQ="${MEM_REQ:-200Gi}"
    BAD=(003 006 013 043 056 057 090 097 101 102 104 105 108) ;;
  *) echo "unknown CLUSTER=$CLUSTER (expected research-common|boogiebonjour; turbox is retired)" >&2; exit 1 ;;
esac
export KUBECONFIG="$KUBECONFIG_FILE"

NAME="${1:?usage: make_eval_job.sh <name> <base-model> <run-ids> [ckpts]}"
BASE="${2:?base model}"
RUNS="${3:?run ids}"
CKPTS="${4:-final}"
MODE="${MODE:-score}"
WITH_PREAMBLE="${WITH_PREAMBLE:-0}"
PREAMBLE_MODE="${PREAMBLE_MODE:-tail}"
FRAME_HINT="${FRAME_HINT:-0}"
ROUNDS="${ROUNDS:-2}"
GEN_VERSIONS="${GEN_VERSIONS:-eac5 rel0}"   # MODE=gen only
NO_GOLD_VERSIONS="${NO_GOLD_VERSIONS:-}"   # zero-shot docs, shipped via configmap
SWEEP="${SWEEP:-0}"                        # run the unsat/vacuous sweep
REPAIR_ROUNDS="${REPAIR_ROUNDS:-0}"        # MODE=gen only
SAMPLES="${SAMPLES:-0}"
TEMPERATURE="${TEMPERATURE:-0.8}"
OUT_TAG="${OUT_TAG:-}"
# Must match the checkpoint's training dataset/prompt pair. Defaults reproduce
# every run scored so far.
DATASET_DIR="${DATASET_DIR:-dataset_clean}"
PROMPT_VARIANT="${PROMPT_VARIANT:-v3}"
JOB="de2-rl-test-$NAME"
CM="de2-rl-test-$NAME-entry"

# Sampling k times multiplies the Verus work, and Verus is the slow half. More
# GPUs would not help; more CPU for parallel checks does.
# Sampling k times multiplies the Verus work and Verus is the slow half, so the
# CPU shape follows the mode. gen+repair is the exception: it compiles one spec at
# a time inside the generation loop, so its Verus work is serial and 16 CPU is
# dead weight -- which matters when a shared node has 6 CPU left and the request
# is the only thing keeping the pod Pending. Overridable either way.
# 2 was the inherited default and is more than any model here needs: the largest
# is a 9B at ~18GB in bf16, which fits on one H100 with room to spare. It costs
# nothing when the cluster is empty and blocks scheduling when it is not --
# turbox sat at 33/32 GPUs requested while this job waited for a second card it
# would not have used.
GPUS="${GPUS:-2}"
if [ "$SAMPLES" -gt 0 ]; then
  : "${CPU_REQ:=32}"; : "${CPU_LIM:=64}"; : "${JOBS:=16}"
elif [ "$MODE" = "gen" ]; then
  : "${CPU_REQ:=4}";  : "${CPU_LIM:=16}"; : "${JOBS:=1}"
else
  : "${CPU_REQ:=16}"; : "${CPU_LIM:=32}"; : "${JOBS:=8}"
fi

# Qwen3.5 needs transformers 5.x; Qwen3 works on either. Using "new" for both
# would be simpler but changes the attention path for the 4B runs, so keep each
# family on the stack it trained with.
case "$BASE" in *3.5*) DEPS=new ;; *) DEPS=ngc ;; esac

bad_values() { for n in "${BAD[@]:-}"; do [[ -n "$n" ]] && echo "                - boogiebonjour-$n.cloud.together.ai"; done; }

# Omitted entirely rather than emitted with an empty `values:`, which the API
# server rejects.
affinity_block() {
  [[ ${#BAD[@]} -eq 0 ]] && return 0
  cat <<EOF
      affinity:
        nodeAffinity:
          requiredDuringSchedulingIgnoredDuringExecution:
            nodeSelectorTerms:
            - matchExpressions:
              - key: kubernetes.io/hostname
                operator: NotIn
                values:
$(bad_values)
EOF
}

# A no-gold version has no (sections, gold) pair in the data repo, so its input
# rides in this ConfigMap instead. Tar-then-base64 rather than --from-file per
# section: 22 files would be 22 keys, and the unpack has to land them under
# training-dataset/ with the layout dataset_loader expects anyway.
ZS_ARG=()
if [ -n "$NO_GOLD_VERSIONS" ]; then
  ZS_TGZ="$(mktemp -t zeroshot).tgz"
  ZS_B64="$(mktemp -t zeroshotb64)"
  PATHS=()
  for v in $NO_GOLD_VERSIONS; do
    for d in "training-dataset/sections/$v" "training-dataset/specs/$v"; do
      [ -d "$REPO_ROOT/$d" ] || { echo "missing $d -- stage it first" >&2; exit 1; }
      PATHS+=("$d")
    done
  done
  # COPYFILE_DISABLE, or macOS bsdtar adds an AppleDouble "._<name>" sidecar for
  # every file carrying an extended attribute. They unpack as real files next to
  # the sections, and list_commands() counts anything ending in _command.txt --
  # so a 22-command document arrived in the pod as 44, and the model spent half
  # the run generating specs for binary resource forks named ._CPU_ON.
  COPYFILE_DISABLE=1 tar czf "$ZS_TGZ" -C "$REPO_ROOT" "${PATHS[@]}"
  base64 < "$ZS_TGZ" > "$ZS_B64"
  # 1MiB is the hard ConfigMap ceiling and base64 costs 33%; fail loudly rather
  # than letting the API server reject a job that took a minute to assemble.
  sz=$(wc -c < "$ZS_B64" | tr -d ' ')
  [ "$sz" -lt 900000 ] || { echo "zero-shot bundle is ${sz}B, too big for a ConfigMap" >&2; exit 1; }
  echo "==> zero-shot bundle: $NO_GOLD_VERSIONS (${sz}B base64)"
  ZS_ARG=(--from-file=zeroshot.tgz.b64="$ZS_B64")
fi

echo "==> configmap $CM"
"$KUBECTL" create configmap "$CM" -n "$NS" \
  "${ZS_ARG[@]}" \
  --from-file=psci_sweep.py="$REPO_ROOT/scripts/psci_sweep.py" \
  --from-file=de2_entrypoint.sh="$REPO_ROOT/k8s/entrypoint_eval.sh" \
  --from-file=eval_checkpoint.py="$REPO_ROOT/scripts/eval_checkpoint.py" \
  --from-file=repair_eval.py="$REPO_ROOT/scripts/repair_eval.py" \
  --from-file=gen_specs.py="$REPO_ROOT/scripts/gen_specs.py" \
  --from-file=dataset_loader.py="$REPO_ROOT/prompt_engineering/dataset_loader.py" \
  --from-file=verify_generated_verus.py="$REPO_ROOT/prompt_engineering/verify_generated_verus.py" \
  --from-file=prompt_engineering_v3.py="$REPO_ROOT/prompt_engineering/prompt_engineering_v3.py" \
  --from-file=prompt_engineering.py="$REPO_ROOT/prompt_engineering/prompt_engineering.py" \
  --dry-run=client -o yaml | "$KUBECTL" apply --server-side --force-conflicts -f - >/dev/null
# Server-side, because client-side apply stores the entire object in the
# kubectl.kubernetes.io/last-applied-configuration annotation, and annotations
# are capped at 256KiB. A zero-shot bundle carrying the reference closure is
# ~196KB base64 and pushed the ConfigMap past it -- the data fits fine, the
# bookkeeping copy of it does not. Server-side apply keeps no such copy.

# A PVC still Terminating from a previous run makes `apply` a silent no-op: it
# sees the object present and reports "unchanged", the delete then completes, and
# the Job is left referencing a volume that does not exist. The pod sits Pending
# with "persistentvolumeclaim not found" and nothing says why. Wait it out first.
if "$KUBECTL" get pvc "${JOB}-work" -n "$NS" >/dev/null 2>&1; then
  if [ -n "$("$KUBECTL" get pvc "${JOB}-work" -n "$NS" -o jsonpath='{.metadata.deletionTimestamp}' 2>/dev/null)" ]; then
    echo "==> waiting for the previous ${JOB}-work to finish deleting"
    for _ in $(seq 1 60); do
      "$KUBECTL" get pvc "${JOB}-work" -n "$NS" >/dev/null 2>&1 || break
      sleep 2
    done
  fi
fi

echo "==> job $JOB  ($BASE, runs: $RUNS, ckpts: $CKPTS, deps: $DEPS)"
cat <<YAML | "$KUBECTL" apply -f -
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: ${JOB}-work
  namespace: ${NS}
  labels: {owner: de2, task: de2-rl-test-eval2, run: ${NAME}}
spec:
  accessModes: [${ACCESS_MODE}]
  storageClassName: ${STORAGE_CLASS}
  resources: {requests: {storage: 400Gi}}
---
apiVersion: batch/v1
kind: Job
metadata:
  name: ${JOB}
  namespace: ${NS}
  labels: {owner: de2, task: de2-rl-test-eval2, run: ${NAME}}
spec:
  # 20, not 3. A Job that reaches BackoffLimitExceeded is terminally Failed and
  # raising the limit afterwards does NOT revive it — bok-0, bok-1 and seed-4b
  # each burned three attempts on an unrelated outage (an HF 403 the client
  # reported as a connection error) and had to be recreated from scratch. The
  # limit needs to be generous BEFORE anything goes wrong.
  backoffLimit: 20
  completions: 1
  template:
    metadata:
      labels: {owner: de2, task: de2-rl-test-eval2, run: ${NAME}}
    spec:
      restartPolicy: Never
      runtimeClassName: nvidia
      hostIPC: true
$(affinity_block)      containers:
      - name: main
        image: nvcr.io/nvidia/pytorch:25.01-py3
        command: ["bash", "-lc", "mkdir -p /work/code/scripts /work/code/prompt_engineering && cp /entry/eval_checkpoint.py /entry/repair_eval.py /entry/gen_specs.py /entry/psci_sweep.py /work/code/scripts/ && cp /entry/dataset_loader.py /entry/verify_generated_verus.py /entry/prompt_engineering_v3.py /entry/prompt_engineering.py /work/code/prompt_engineering/ && bash /entry/de2_entrypoint.sh"]
        securityContext: {privileged: true}
        resources:
          limits:   {cpu: "${CPU_LIM}", memory: ${MEM_LIM}, nvidia.com/gpu: ${GPUS}}
          requests: {cpu: "${CPU_REQ}", memory: ${MEM_REQ}, nvidia.com/gpu: ${GPUS}}
        env:
        - {name: RUN_IDS,     value: "${RUNS}"}
        - {name: BASE_MODEL,  value: "${BASE}"}
        - {name: CKPTS,       value: "${CKPTS}"}
        - {name: DEPS,        value: "${DEPS}"}
        - {name: MODE,        value: "${MODE}"}
        - {name: WITH_PREAMBLE, value: "${WITH_PREAMBLE}"}
        - {name: PREAMBLE_MODE, value: "${PREAMBLE_MODE}"}
        - {name: FRAME_HINT,    value: "${FRAME_HINT}"}
        - {name: ROUNDS,      value: "${ROUNDS}"}
        - {name: GEN_VERSIONS,  value: "${GEN_VERSIONS}"}
        - {name: NO_GOLD_VERSIONS, value: "${NO_GOLD_VERSIONS}"}
        - {name: EXPECT_SECTIONS, value: "${EXPECT_SECTIONS:-}"}
        - {name: SWEEP,         value: "${SWEEP}"}
        - {name: REPAIR_ROUNDS, value: "${REPAIR_ROUNDS}"}
        - {name: SAMPLES,     value: "${SAMPLES}"}
        - {name: TEMPERATURE, value: "${TEMPERATURE}"}
        - {name: OUT_TAG,     value: "${OUT_TAG}"}
        - {name: DATASET_DIR,    value: "${DATASET_DIR}"}
        - {name: PROMPT_VARIANT, value: "${PROMPT_VARIANT}"}
        - {name: JOBS,        value: "${JOBS}"}
        - {name: HOME,       value: /work/home}
        - {name: HF_HOME,    value: /work/hf-cache}
        - {name: HF_TOKEN,     valueFrom: {secretKeyRef: {name: de2-rl-test-hf, key: token}}}
        - {name: HF_CKPT_REPO, valueFrom: {secretKeyRef: {name: de2-rl-test-hf, key: ckpt_repo}}}
        volumeMounts:
        - {name: work,  mountPath: /work}
        - {name: dshm,  mountPath: /dev/shm}
        - {name: entry, mountPath: /entry}
      volumes:
      - {name: work,  persistentVolumeClaim: {claimName: ${JOB}-work}}
      - {name: dshm,  emptyDir: {medium: Memory, sizeLimit: 64Gi}}
      - {name: entry, configMap: {name: ${CM}, defaultMode: 493}}
YAML
