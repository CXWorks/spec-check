#!/usr/bin/env bash
# Render and submit the sft2-* training Jobs.
#
#   k8s/make_jobs.sh --dry-run     # print manifests
#   k8s/make_jobs.sh               # submit all
#   k8s/make_jobs.sh sft2-0 sft2-2 # submit a subset
#
# Object names carry no project information; the run id is the join key across
# k8s, W&B, and HF. See docs/gpu-and-runs.md.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# KUBECONFIG is chosen by the cluster profile below; an explicitly exported
# KUBECONFIG still wins, via KUBECONFIG_FILE.
KUBECONFIG_FILE="${KUBECONFIG_FILE:-${KUBECONFIG:-}}"
KUBECTL="${KUBECTL:-kubectl}"
NS=default
CM=de2-rl-test-sft2-entry

# Which built dataset in the HF data repo to train on. dataset_clean is the
# shipped split every sft2-* run so far used. dataset_bench additionally holds
# out the commands the bug-finding benchmarks score, without which those
# benchmarks cannot measure a fine-tuned model at all — it was trained on the
# answers (training-dataset/benchmark_commands.json). Whatever is chosen here,
# the eval job needs the SAME DATASET_DIR and the prompt variant recorded in
# that dataset's splits.json.
DATASET_DIR="${DATASET_DIR:-dataset_clean}"
echo "==> dataset: $DATASET_DIR"

# Cluster profile. Defaults are boogiebonjour's, so an unqualified run is
# unchanged. `CLUSTER=turbox` targets turbox-h100, which differs in ways that
# are not cosmetic:
#   - shared-wekafs is a real shared filesystem (RWX), not node-local. That
#     removes the whole local-path failure mode: PVCs no longer pin a replacement
#     pod to the node that just failed, and rescuing a checkpoint no longer needs
#     one helper pod per node.
#   - its GPU nodes have 63.4 CPU / 723Gi, so boogiebonjour's 96 CPU / 900Gi
#     limits would never schedule. This is a hard failure, not a slow one.
CLUSTER="${CLUSTER:-boogiebonjour}"
case "$CLUSTER" in
  turbox)
    : "${KUBECONFIG_FILE:=$HOME/.kube/configs/turbox-h100.yaml}"
    STORAGE_CLASS="${STORAGE_CLASS:-shared-wekafs}"
    ACCESS_MODE="${ACCESS_MODE:-ReadWriteMany}"
    CPU_LIM="${CPU_LIM:-60}";  CPU_REQ="${CPU_REQ:-32}"
    MEM_LIM="${MEM_LIM:-700Gi}"; MEM_REQ="${MEM_REQ:-400Gi}"
    SHM="${SHM:-128Gi}"
    BAD=()                     # no known-bad node list for this cluster
    ;;
  boogiebonjour)
    : "${KUBECONFIG_FILE:=$HOME/.kube/boogiebonjour}"
    STORAGE_CLASS="${STORAGE_CLASS:-local-path}"
    ACCESS_MODE="${ACCESS_MODE:-ReadWriteOnce}"
    CPU_LIM="${CPU_LIM:-96}";  CPU_REQ="${CPU_REQ:-56}"
    MEM_LIM="${MEM_LIM:-900Gi}"; MEM_REQ="${MEM_REQ:-512Gi}"
    SHM="${SHM:-256Gi}"
    ;;
  *) echo "unknown CLUSTER=$CLUSTER (expected boogiebonjour|turbox)" >&2; exit 1 ;;
esac
export KUBECONFIG="$KUBECONFIG_FILE"
echo "==> cluster: $CLUSTER  storage: $STORAGE_CLASS ($ACCESS_MODE)"

# run-id | base model | precision | method | deps profile (see entrypoint.sh) | seed
#
# The -s#### runs are seed replicates, not new configurations: same data, same
# hyperparameters, only initialisation and batch order differ. They exist because
# a single run gives no way to tell a real effect from run-to-run noise — the
# 40-command eval set can only resolve a ~22pp difference from one run each, and
# no affordable change to the split fixes that (docs/dataset.md). Replicates buy
# the same resolution for GPU time instead of training data.
#
# The seed varies ONLY training. The held-out split is pinned by SPLIT_SEED in
# build_dataset.py; if it moved with this, the replicates would be measured on
# different test sets and would not be comparable.
RUNS=(
  "sft2-0|Qwen/Qwen3-4B|bf16|lora|ngc|42"
  "sft2-1|Qwen/Qwen3-4B|fp16|lora|ngc|42"
  "sft2-2|Qwen/Qwen3.5-9B|bf16|lora|new|42"
  "sft2-3|Qwen/Qwen3-4B|bf16|full|ngc|42"
  "sft2-4|Qwen/Qwen3.5-9B|bf16|full|new|42"
  "sft2-0-s1337|Qwen/Qwen3-4B|bf16|lora|ngc|1337"
  "sft2-0-s2024|Qwen/Qwen3-4B|bf16|lora|ngc|2024"
  "sft2-1-s1337|Qwen/Qwen3-4B|fp16|lora|ngc|1337"
  "sft2-1-s2024|Qwen/Qwen3-4B|fp16|lora|ngc|2024"
  "sft2-3-s1337|Qwen/Qwen3-4B|bf16|full|ngc|1337"
  "sft2-3-s2024|Qwen/Qwen3-4B|bf16|full|ngc|2024"

  # sft3-*: same configurations as sft2-0 and sft2-2, retrained on dataset_bench
  # (DATASET_DIR=dataset_bench). The index is kept aligned with sft2 so the pair
  # to compare is obvious. These are the first runs that can be scored on the
  # bug-finding benchmarks at all -- every sft2-* was trained on the commands
  # those benchmarks check.
  "sft3-0|Qwen/Qwen3-4B|bf16|lora|ngc|42"
  "sft3-2|Qwen/Qwen3.5-9B|bf16|lora|new|42"

  # Seed replicates on dataset_bench. Every sft3 comparison so far came back
  # "not significant" -- 4B vs 9B p=0.454, new vs old p=0.344/0.625 -- because a
  # 49-command set cannot resolve a 5-10pp difference from one run each. The
  # split is pinned by SPLIT_SEED, so replicates share a test set and only
  # initialisation and batch order differ.
  "sft3-0-s1337|Qwen/Qwen3-4B|bf16|lora|ngc|1337"
  "sft3-0-s2024|Qwen/Qwen3-4B|bf16|lora|ngc|2024"
  "sft3-2-s1337|Qwen/Qwen3.5-9B|bf16|lora|new|1337"
  "sft3-2-s2024|Qwen/Qwen3.5-9B|bf16|lora|new|2024"
)

# Nodes boogiebonjour's own production Jobs avoid. Reused rather than
# rediscovered. Set above per cluster profile; turbox has none.
[[ "$CLUSTER" == "boogiebonjour" ]] && BAD=(003 006 013 043 056 057 090 097 101 102 104 105 108)

DRY=""
[[ "${1:-}" == "--dry-run" ]] && { DRY=1; shift; }
WANTED=("$@")

want() {
  [[ ${#WANTED[@]} -eq 0 ]] && return 0
  for w in "${WANTED[@]}"; do [[ "$w" == "$1" ]] && return 0; done
  return 1
}

# 16 spaces: these are items of `values:`, which sits at 16 in the template below.
# At 12 they parse as siblings of nodeSelectorTerms and the API rejects the Job
# with a confusing "cannot unmarshal string into NodeSelectorTerm".
bad_values() { for n in "${BAD[@]:-}"; do [[ -n "$n" ]] && echo "                - boogiebonjour-$n.cloud.together.ai"; done; }

# An `affinity:` with an empty `values:` list is rejected by the API server, so
# the whole block is omitted rather than emitted empty when there are no bad nodes.
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

if [[ -z "$DRY" ]]; then
  echo "==> configmap $CM (train.py + entrypoint)"
  "$KUBECTL" create configmap "$CM" -n "$NS" \
    --from-file=train.py="$REPO_ROOT/training/train.py" \
    --from-file=de2_entrypoint.sh="$REPO_ROOT/k8s/entrypoint.sh" \
    --dry-run=client -o yaml | "$KUBECTL" apply -f - >/dev/null
fi

for spec in "${RUNS[@]}"; do
  IFS='|' read -r RUN MODEL PREC METHOD DEPS SEED <<<"$spec"
  SEED="${SEED:-42}"
  want "$RUN" || continue
  JOB="de2-rl-test-$RUN"

  # Full fine-tunes need optimizer state for every parameter, so they get a
  # bigger PVC and a smaller per-device batch.
  if [[ "$METHOD" == "full" ]]; then SIZE=800Gi; BATCH=1; else SIZE=400Gi; BATCH=1; fi

  MANIFEST=$(cat <<YAML
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: ${JOB}-work
  namespace: ${NS}
  labels: {owner: de2, task: de2-rl-test-sft2, run: ${RUN}}
spec:
  accessModes: [${ACCESS_MODE}]
  storageClassName: ${STORAGE_CLASS}
  resources: {requests: {storage: ${SIZE}}}
---
apiVersion: batch/v1
kind: Job
metadata:
  name: ${JOB}
  namespace: ${NS}
  labels: {owner: de2, task: de2-rl-test-sft2, run: ${RUN}}
spec:
  backoffLimit: 2
  completions: 1
  template:
    metadata:
      labels: {owner: de2, task: de2-rl-test-sft2, run: ${RUN}}
    spec:
      restartPolicy: Never
      runtimeClassName: nvidia
      hostIPC: true
$(affinity_block)      containers:
      - name: main
        image: nvcr.io/nvidia/pytorch:25.01-py3
        command: ["bash", "/entry/de2_entrypoint.sh"]
        securityContext: {privileged: true}
        resources:
          limits:   {cpu: "${CPU_LIM}", memory: ${MEM_LIM}, nvidia.com/gpu: 8}
          requests: {cpu: "${CPU_REQ}", memory: ${MEM_REQ}, nvidia.com/gpu: 8}
        env:
        - {name: RUN_ID,     value: "${RUN}"}
        - {name: BASE_MODEL, value: "${MODEL}"}
        - {name: PRECISION,  value: "${PREC}"}
        - {name: METHOD,     value: "${METHOD}"}
        - {name: DEPS,       value: "${DEPS}"}
        - {name: SEED,       value: "${SEED}"}
        - {name: EPOCHS,     value: "3"}
        - {name: DATASET_DIR, value: "${DATASET_DIR}"}
        - {name: BATCH,      value: "${BATCH}"}
        - {name: HOME,       value: /work/home}
        - {name: HF_HOME,    value: /work/hf-cache}
        - {name: HF_TOKEN,      valueFrom: {secretKeyRef: {name: de2-rl-test-hf,    key: token}}}
        - {name: HF_CKPT_REPO,  valueFrom: {secretKeyRef: {name: de2-rl-test-hf,    key: ckpt_repo}}}
        - {name: WANDB_API_KEY, valueFrom: {secretKeyRef: {name: de2-rl-test-wandb, key: token}}}
        - {name: WANDB_PROJECT, valueFrom: {secretKeyRef: {name: de2-rl-test-wandb, key: project}}}
        - {name: WANDB_ENTITY,  valueFrom: {secretKeyRef: {name: de2-rl-test-wandb, key: entity}}}
        volumeMounts:
        - {name: work,  mountPath: /work}
        - {name: dshm,  mountPath: /dev/shm}
        - {name: entry, mountPath: /entry}
      volumes:
      - {name: work,  persistentVolumeClaim: {claimName: ${JOB}-work}}
      - {name: dshm,  emptyDir: {medium: Memory, sizeLimit: ${SHM}}}
      - {name: entry, configMap: {name: ${CM}, defaultMode: 493}}
YAML
)

  if [[ -n "$DRY" ]]; then
    echo "$MANIFEST"
  else
    echo "==> submitting $JOB  ($MODEL $PREC $METHOD seed=$SEED)"
    echo "$MANIFEST" | "$KUBECTL" apply -f - | sed 's/^/    /'
  fi
done
