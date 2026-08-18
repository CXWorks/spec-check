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
export KUBECONFIG="${KUBECONFIG:-$HOME/.kube/boogiebonjour}"
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
)

# Nodes the cluster's own production Jobs avoid. Reused rather than rediscovered.
BAD=(003 006 013 043 056 057 090 097 101 102 104 105 108)

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
bad_values() { for n in "${BAD[@]}"; do echo "                - boogiebonjour-$n.cloud.together.ai"; done; }

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
  accessModes: [ReadWriteOnce]
  storageClassName: local-path
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
      affinity:
        nodeAffinity:
          requiredDuringSchedulingIgnoredDuringExecution:
            nodeSelectorTerms:
            - matchExpressions:
              - key: kubernetes.io/hostname
                operator: NotIn
                values:
$(bad_values)
      containers:
      - name: main
        image: nvcr.io/nvidia/pytorch:25.01-py3
        command: ["bash", "/entry/de2_entrypoint.sh"]
        securityContext: {privileged: true}
        resources:
          limits:   {cpu: "96", memory: 900Gi, nvidia.com/gpu: 8}
          requests: {cpu: "56", memory: 512Gi, nvidia.com/gpu: 8}
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
      - {name: dshm,  emptyDir: {medium: Memory, sizeLimit: 256Gi}}
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
