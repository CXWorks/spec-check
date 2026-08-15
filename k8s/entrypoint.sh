#!/bin/bash
# Training Job entrypoint. Mounted from a ConfigMap alongside train.py.
#
# Everything a run needs comes from the network, not from the node: the dataset
# is pulled from the HF dataset repo and the result is pushed back. local-path
# PVCs are node-local, so anything left only on the PVC dies with the node.
set -euo pipefail

: "${RUN_ID:?RUN_ID is required}"
: "${BASE_MODEL:?BASE_MODEL is required}"
PRECISION="${PRECISION:-bf16}"
METHOD="${METHOD:-lora}"
EPOCHS="${EPOCHS:-3}"
MAX_SEQ="${MAX_SEQ:-12288}"
BATCH="${BATCH:-1}"
GRAD_ACCUM="${GRAD_ACCUM:-4}"
LR="${LR:-2e-4}"
DATA_REPO="${DATA_REPO:-jisenli/spec-check-data}"
EXTRA_ARGS="${EXTRA_ARGS:-}"

echo "[entry] run=$RUN_ID model=$BASE_MODEL precision=$PRECISION method=$METHOD"

# Pinned deliberately. transformers 5.x passes `in_order` to DataLoader, which the
# NGC image's torch 2.6.0a0 does not accept; upgrading torch instead would break
# the NGC-built flash-attn. See docs/gpu-and-runs.md.
echo "[entry] installing deps"
python -m pip install --no-cache-dir -q \
  "transformers==4.57.1" "trl==0.24.0" "peft==0.17.1" \
  datasets accelerate wandb huggingface_hub

mkdir -p /work/data /work/out
echo "[entry] fetching dataset from $DATA_REPO"
python - <<PY
from huggingface_hub import snapshot_download
import os, shutil
p = snapshot_download(repo_id="${DATA_REPO}", repo_type="dataset",
                      token=os.environ.get("HF_TOKEN"),
                      allow_patterns=["dataset_clean/*"])
shutil.copytree(os.path.join(p, "dataset_clean"), "/work/data/dataset_clean",
                dirs_exist_ok=True)
print("[entry] dataset ready")
PY
wc -l /work/data/dataset_clean/*.jsonl

# Probe wandb before training rather than discovering it at on_train_begin: a
# bad entity raises there and kills the run after the model is loaded and the
# data masked. A logging backend must not be able to lose a training run.
if [ -n "${WANDB_API_KEY:-}" ]; then
  if python - <<'PY'
import os, sys, urllib.request, json, base64
ent = os.environ.get("WANDB_ENTITY") or ""
if not ent:
    sys.exit(0)  # let wandb pick its default
req = urllib.request.Request(
    "https://api.wandb.ai/graphql",
    data=json.dumps({"query": '{entity(name:"%s"){id}}' % ent}).encode(),
    headers={"Content-Type": "application/json",
             "Authorization": "Basic " + base64.b64encode(
                 ("api:" + os.environ["WANDB_API_KEY"]).encode()).decode()})
try:
    d = json.load(urllib.request.urlopen(req, timeout=20))
    e = (d.get("data") or {}).get("entity")
    # id decodes to "Entity:-1" when the name does not resolve; a non-null
    # object alone is not proof the entity exists.
    ok = bool(e) and base64.b64decode(e["id"]).decode() != "Entity:-1"
    sys.exit(0 if ok else 1)
except Exception:
    sys.exit(1)
PY
  then
    echo "[entry] wandb: ${WANDB_ENTITY:-<default>}/${WANDB_PROJECT:-<default>} run=$RUN_ID"
  else
    echo "[entry] WARNING: wandb entity '${WANDB_ENTITY}' unusable — training without it"
    EXTRA_ARGS="$EXTRA_ARGS --no-wandb"
  fi
else
  EXTRA_ARGS="$EXTRA_ARGS --no-wandb"
fi

NPROC="$(python -c 'import torch;print(torch.cuda.device_count())')"
echo "[entry] launching on $NPROC GPUs"

torchrun --nproc_per_node="$NPROC" /entry/train.py \
  --train /work/data/dataset_clean/train.jsonl \
  --val   /work/data/dataset_clean/val.jsonl \
  --out   "/work/out/$RUN_ID" \
  --run-id "$RUN_ID" \
  --model "$BASE_MODEL" \
  --precision "$PRECISION" \
  --method "$METHOD" \
  --epochs "$EPOCHS" \
  --max-seq "$MAX_SEQ" \
  --batch-size "$BATCH" \
  --grad-accum "$GRAD_ACCUM" \
  --lr "$LR" \
  --push $EXTRA_ARGS

echo "[entry] done: $RUN_ID"
