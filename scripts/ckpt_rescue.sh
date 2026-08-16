#!/usr/bin/env bash
# Inventory or re-upload the adapters sitting on the training PVCs.
#
#   scripts/ckpt_rescue.sh verify     # what is on the PVCs, per run
#   scripts/ckpt_rescue.sh upload     # push them to $HF_CKPT_REPO
#   scripts/ckpt_rescue.sh clean      # remove the helper pods
#
# `local-path` PVCs are node-local, so a pod can only see the volumes on its own
# node — hence one helper pod per node rather than one that mounts everything.
#
# Why this exists: the account's private storage quota was exhausted, which makes
# HF answer 403 on every read, so the checkpoints there became unreachable. The
# PVCs are the copy that still works, which is what makes deleting the HF repo
# safe rather than lossy: it is the inaccessible copy being removed.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."
export KUBECONFIG="${KUBECONFIG:-$HOME/.kube/boogiebonjour}"
NS=default
MODE="${1:?usage: ckpt_rescue.sh verify|upload|clean}"

# run : node : pvc — the pairing is fixed at provisioning time and is what
# `kubectl get pvc -o custom-columns=...selected-node` reports.
RUNS=(
  "sft2-0:080:de2-rl-test-sft2-0-work"
  "sft2-1-s2024:080:de2-rl-test-sft2-1-s2024-work"
  "sft2-1:064:de2-rl-test-sft2-1-work"
  "sft2-2:100:de2-rl-test-sft2-2-work"
  "sft2-0-s1337:118:de2-rl-test-sft2-0-s1337-work"
  "sft2-0-s2024:049:de2-rl-test-sft2-0-s2024-work"
  "sft2-1-s1337:015:de2-rl-test-sft2-1-s1337-work"
  "sft2-3:121:de2-rl-test-sft2-3-work"
  "sft2-3-s1337:093:de2-rl-test-sft2-3-s1337-work"
  "sft2-3-s2024:071:de2-rl-test-sft2-3-s2024-work"
)

# Full fine-tunes upload their final weights only — no per-epoch checkpoints and
# no optimizer state. That is 8GB each rather than 75GB, and 75GB apiece is what
# exhausted the account quota the first time. `final` is still worth having:
# truncation hit these runs twice as often as the LoRA ones, which forced the
# "full fine-tuning is worse" conclusion to be withdrawn, so the comparison is
# currently unresolved rather than settled.
FINAL_ONLY="sft2-3 sft2-3-s1337 sft2-3-s2024"

nodes() { for r in "${RUNS[@]}"; do echo "${r#*:}" | cut -d: -f1; done | sort -u; }
pvcs_on() { for r in "${RUNS[@]}"; do
  IFS=: read -r _ n p <<<"$r"; [ "$n" = "$1" ] && echo "$p"; done; }

if [ "$MODE" = "clean" ]; then
  kubectl delete pod -n "$NS" -l task=de2-rl-test-rescue --ignore-not-found
  exit 0
fi

IMAGE=busybox:1.36
CMD='sleep 7200'
if [ "$MODE" = "upload" ]; then
  IMAGE=python:3.11-slim
  CMD='sleep 7200'
fi

for node in $(nodes); do
  pod="de2-rl-test-rescue-$node"
  kubectl get pod -n "$NS" "$pod" >/dev/null 2>&1 && continue
  # Built with real newlines rather than \n inside a printf argument: printf
  # expands escapes in the FORMAT string, not in %s substitutions, so the
  # backslash-n would reach the API server literally and fail to parse.
  mounts=""; vols=""; i=0
  while read -r pvc; do
    i=$((i+1))
    mounts+="        - {name: v$i, mountPath: /mnt/v$i}"$'\n'
    vols+="      - {name: v$i, persistentVolumeClaim: {claimName: $pvc}}"$'\n'
  done < <(pvcs_on "$node")
  cat <<YAML | kubectl apply -f - >/dev/null
apiVersion: v1
kind: Pod
metadata:
  name: ${pod}
  namespace: ${NS}
  labels: {owner: de2, task: de2-rl-test-rescue}
spec:
  restartPolicy: Never
  nodeName: boogiebonjour-${node}.cloud.together.ai
  containers:
  - name: main
    image: ${IMAGE}
    command: ["sh","-c","${CMD}"]
    # Tiny requests on purpose: these land on nodes already running 8-GPU
    # training jobs, and node 064 had 819m of 72000m CPU left. A 1-CPU request
    # is not schedulable there, and the work here is a file copy.
    resources: {limits: {cpu: "2", memory: 8Gi}, requests: {cpu: "100m", memory: 256Mi}}
    env:
    - {name: HF_TOKEN,     valueFrom: {secretKeyRef: {name: de2-rl-test-hf, key: token}}}
    - {name: HF_CKPT_REPO, valueFrom: {secretKeyRef: {name: de2-rl-test-hf, key: ckpt_repo}}}
    volumeMounts:
${mounts}  volumes:
${vols}
YAML
  echo "  pod $pod"
done

echo "==> waiting for helper pods"
for node in $(nodes); do
  kubectl wait --for=condition=Ready "pod/de2-rl-test-rescue-$node" -n "$NS" --timeout=300s >/dev/null
done

if [ "$MODE" = "verify" ]; then
  printf '\n%-16s %-6s %-10s %-22s %s\n' RUN NODE SIZE ADAPTER CHECKPOINTS
  for r in "${RUNS[@]}"; do
    IFS=: read -r run node _ <<<"$r"
    kubectl exec -n "$NS" "de2-rl-test-rescue-$node" -- sh -c "
      d=\$(ls -d /mnt/v*/out/$run 2>/dev/null | head -1)
      [ -z \"\$d\" ] && { printf '%-16s %-6s %s\n' '$run' '$node' 'MISSING'; exit 0; }
      sz=\$(du -sh \"\$d\" 2>/dev/null | cut -f1)
      a=\$([ -f \"\$d/adapter_model.safetensors\" ] && echo adapter || \
           ([ -f \"\$d/model.safetensors.index.json\" ] && echo full-model || echo NONE))
      ck=\$(ls -d \"\$d\"/checkpoint-* 2>/dev/null | wc -l)
      printf '%-16s %-6s %-10s %-22s %s\n' '$run' '$node' \"\$sz\" \"\$a\" \"\$ck ckpts\"
    "
  done
  exit 0
fi

# ---- upload ----
: "${HF_CKPT_REPO:?set HF_CKPT_REPO}"
ONLY="${ONLY:-}"     # restrict to these runs, e.g. ONLY='sft2-3 sft2-3-s1337'
echo "==> uploading to $HF_CKPT_REPO (final-only: $FINAL_ONLY)"
for r in "${RUNS[@]}"; do
  IFS=: read -r run node _ <<<"$r"
  if [ -n "$ONLY" ]; then
    case " $ONLY " in *" $run "*) ;; *) continue;; esac
  fi
  fin=0
  case " $FINAL_ONLY " in *" $run "*) fin=1;; esac
  echo "  == $run (node $node)$([ $fin = 1 ] && echo ' [final only]')"
  kubectl exec -n "$NS" "de2-rl-test-rescue-$node" -- env HF_FINAL_ONLY="$fin" sh -c "
    python -c 'import huggingface_hub' 2>/dev/null || pip install -q huggingface_hub 2>&1 | tail -1
    d=\$(ls -d /mnt/v*/out/$run 2>/dev/null | head -1)
    [ -z \"\$d\" ] && { echo '     MISSING - skipped'; exit 0; }
    HF_SRC=\"\$d\" HF_RUN=$run python - <<'PY'
import os
from huggingface_hub import HfApi
api = HfApi(token=os.environ['HF_TOKEN'])
run, src = os.environ['HF_RUN'], os.environ['HF_SRC']
# Same two filters push_to_hub needed: the run directory CONTAINS its per-epoch
# checkpoints, so uploading it whole would nest a second copy of each under
# final/, and ignore_patterns matches relative paths so optimizer.pt needs **/.
skip = ['checkpoint-*', 'checkpoint-*/**', '**/optimizer.pt', 'optimizer.pt',
        '**/scheduler.pt', '**/rng_state*.pth']
api.upload_folder(folder_path=src, path_in_repo=f'{run}/final',
                  repo_id=os.environ['HF_CKPT_REPO'], repo_type='model',
                  ignore_patterns=skip)
print(f'     uploaded {run}/final')
for ck in sorted(os.listdir(src)):
    if os.environ.get('HF_FINAL_ONLY') == '1':
        break
    if ck.startswith('checkpoint-'):
        api.upload_folder(folder_path=os.path.join(src, ck), path_in_repo=f'{run}/{ck}',
                          repo_id=os.environ['HF_CKPT_REPO'], repo_type='model',
                          ignore_patterns=skip[2:])
        print(f'     uploaded {run}/{ck}')
PY
  "
done
echo "==> done"
