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
export KUBECONFIG="${KUBECONFIG:-$HOME/.kube/boogiebonjour}"
KUBECTL="${KUBECTL:-kubectl}"
NS=default

NAME="${1:?usage: make_eval_job.sh <name> <base-model> <run-ids> [ckpts]}"
BASE="${2:?base model}"
RUNS="${3:?run ids}"
CKPTS="${4:-final}"
SAMPLES="${SAMPLES:-0}"
TEMPERATURE="${TEMPERATURE:-0.8}"
OUT_TAG="${OUT_TAG:-}"
JOB="de2-rl-test-$NAME"
CM="de2-rl-test-$NAME-entry"

# Sampling k times multiplies the Verus work, and Verus is the slow half. More
# GPUs would not help; more CPU for parallel checks does.
if [ "$SAMPLES" -gt 0 ]; then CPU_REQ=32; CPU_LIM=64; JOBS=16; else CPU_REQ=16; CPU_LIM=32; JOBS=8; fi

# Qwen3.5 needs transformers 5.x; Qwen3 works on either. Using "new" for both
# would be simpler but changes the attention path for the 4B runs, so keep each
# family on the stack it trained with.
case "$BASE" in *3.5*) DEPS=new ;; *) DEPS=ngc ;; esac

BAD=(003 006 013 043 056 057 090 097 101 102 104 105 108)
bad_values() { for n in "${BAD[@]}"; do echo "                - boogiebonjour-$n.cloud.together.ai"; done; }

echo "==> configmap $CM"
"$KUBECTL" create configmap "$CM" -n "$NS" \
  --from-file=de2_entrypoint.sh="$REPO_ROOT/k8s/entrypoint_eval.sh" \
  --from-file=eval_checkpoint.py="$REPO_ROOT/scripts/eval_checkpoint.py" \
  --from-file=dataset_loader.py="$REPO_ROOT/prompt_engineering/dataset_loader.py" \
  --from-file=verify_generated_verus.py="$REPO_ROOT/prompt_engineering/verify_generated_verus.py" \
  --from-file=prompt_engineering_v3.py="$REPO_ROOT/prompt_engineering/prompt_engineering_v3.py" \
  --from-file=prompt_engineering.py="$REPO_ROOT/prompt_engineering/prompt_engineering.py" \
  --dry-run=client -o yaml | "$KUBECTL" apply -f - >/dev/null

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
  accessModes: [ReadWriteOnce]
  storageClassName: local-path
  resources: {requests: {storage: 400Gi}}
---
apiVersion: batch/v1
kind: Job
metadata:
  name: ${JOB}
  namespace: ${NS}
  labels: {owner: de2, task: de2-rl-test-eval2, run: ${NAME}}
spec:
  backoffLimit: 3
  completions: 1
  template:
    metadata:
      labels: {owner: de2, task: de2-rl-test-eval2, run: ${NAME}}
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
        command: ["bash", "-lc", "mkdir -p /work/code/scripts /work/code/prompt_engineering && cp /entry/eval_checkpoint.py /work/code/scripts/ && cp /entry/dataset_loader.py /entry/verify_generated_verus.py /entry/prompt_engineering_v3.py /entry/prompt_engineering.py /work/code/prompt_engineering/ && bash /entry/de2_entrypoint.sh"]
        securityContext: {privileged: true}
        resources:
          limits:   {cpu: "${CPU_LIM}", memory: 400Gi, nvidia.com/gpu: 2}
          requests: {cpu: "${CPU_REQ}", memory: 200Gi, nvidia.com/gpu: 2}
        env:
        - {name: RUN_IDS,     value: "${RUNS}"}
        - {name: BASE_MODEL,  value: "${BASE}"}
        - {name: CKPTS,       value: "${CKPTS}"}
        - {name: DEPS,        value: "${DEPS}"}
        - {name: SAMPLES,     value: "${SAMPLES}"}
        - {name: TEMPERATURE, value: "${TEMPERATURE}"}
        - {name: OUT_TAG,     value: "${OUT_TAG}"}
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
