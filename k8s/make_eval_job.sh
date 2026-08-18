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
# boogiebonjour so existing invocations are unchanged. turbox's GPU nodes have
# 63.4 CPU, so boogiebonjour's 32/64 CPU request for a sampling run is near the
# node ceiling and its 400Gi memory limit is fine, but the storage class is the
# real difference: shared-wekafs is RWX, so an eval pod is not pinned to the node
# its PVC was created on.
CLUSTER="${CLUSTER:-boogiebonjour}"
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
    : "${KUBECONFIG_FILE:=$HOME/.kube/configs/turbox-h100.yaml}"
    STORAGE_CLASS="${STORAGE_CLASS:-shared-wekafs}"
    ACCESS_MODE="${ACCESS_MODE:-ReadWriteMany}"
    # 200Gi was inherited from boogiebonjour, whose nodes are larger. On turbox
    # (723Gi/node) it does not fit beside a 9B training job requesting 400Gi, and
    # the eval does not need it: the biggest model here is 18GB in bf16, the rest
    # is Verus subprocesses. Requesting what is actually used lets an eval run
    # alongside training instead of sitting Pending until the cluster drains.
    MEM_LIM="${MEM_LIM:-200Gi}"; MEM_REQ="${MEM_REQ:-64Gi}"
    BAD=() ;;
  boogiebonjour)
    : "${KUBECONFIG_FILE:=$HOME/.kube/boogiebonjour}"
    STORAGE_CLASS="${STORAGE_CLASS:-local-path}"
    ACCESS_MODE="${ACCESS_MODE:-ReadWriteOnce}"
    MEM_LIM="${MEM_LIM:-400Gi}"; MEM_REQ="${MEM_REQ:-200Gi}"
    BAD=(003 006 013 043 056 057 090 097 101 102 104 105 108) ;;
  *) echo "unknown CLUSTER=$CLUSTER (expected boogiebonjour|turbox|research-common)" >&2; exit 1 ;;
esac
export KUBECONFIG="$KUBECONFIG_FILE"

NAME="${1:?usage: make_eval_job.sh <name> <base-model> <run-ids> [ckpts]}"
BASE="${2:?base model}"
RUNS="${3:?run ids}"
CKPTS="${4:-final}"
MODE="${MODE:-score}"
WITH_PREAMBLE="${WITH_PREAMBLE:-0}"
FRAME_HINT="${FRAME_HINT:-0}"
ROUNDS="${ROUNDS:-2}"
GEN_VERSIONS="${GEN_VERSIONS:-eac5 rel0}"   # MODE=gen only
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

echo "==> configmap $CM"
"$KUBECTL" create configmap "$CM" -n "$NS" \
  --from-file=de2_entrypoint.sh="$REPO_ROOT/k8s/entrypoint_eval.sh" \
  --from-file=eval_checkpoint.py="$REPO_ROOT/scripts/eval_checkpoint.py" \
  --from-file=repair_eval.py="$REPO_ROOT/scripts/repair_eval.py" \
  --from-file=gen_specs.py="$REPO_ROOT/scripts/gen_specs.py" \
  --from-file=dataset_loader.py="$REPO_ROOT/prompt_engineering/dataset_loader.py" \
  --from-file=verify_generated_verus.py="$REPO_ROOT/prompt_engineering/verify_generated_verus.py" \
  --from-file=prompt_engineering_v3.py="$REPO_ROOT/prompt_engineering/prompt_engineering_v3.py" \
  --from-file=prompt_engineering.py="$REPO_ROOT/prompt_engineering/prompt_engineering.py" \
  --dry-run=client -o yaml | "$KUBECTL" apply -f - >/dev/null

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
        command: ["bash", "-lc", "mkdir -p /work/code/scripts /work/code/prompt_engineering && cp /entry/eval_checkpoint.py /entry/repair_eval.py /entry/gen_specs.py /work/code/scripts/ && cp /entry/dataset_loader.py /entry/verify_generated_verus.py /entry/prompt_engineering_v3.py /entry/prompt_engineering.py /work/code/prompt_engineering/ && bash /entry/de2_entrypoint.sh"]
        securityContext: {privileged: true}
        resources:
          limits:   {cpu: "${CPU_LIM}", memory: ${MEM_LIM}, nvidia.com/gpu: 2}
          requests: {cpu: "${CPU_REQ}", memory: ${MEM_REQ}, nvidia.com/gpu: 2}
        env:
        - {name: RUN_IDS,     value: "${RUNS}"}
        - {name: BASE_MODEL,  value: "${BASE}"}
        - {name: CKPTS,       value: "${CKPTS}"}
        - {name: DEPS,        value: "${DEPS}"}
        - {name: MODE,        value: "${MODE}"}
        - {name: WITH_PREAMBLE, value: "${WITH_PREAMBLE}"}
        - {name: FRAME_HINT,    value: "${FRAME_HINT}"}
        - {name: ROUNDS,      value: "${ROUNDS}"}
        - {name: GEN_VERSIONS,  value: "${GEN_VERSIONS}"}
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
