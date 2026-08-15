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

# Two dependency profiles, because the two model families cannot share one.
#
# ngc: keeps the image's torch 2.6.0a0 and its prebuilt flash-attn. transformers
#   must stay on 4.57.x — 5.x passes `in_order` to DataLoader behind an
#   `is_torch_greater_or_equal_than_2_6` gate, and NGC's 2.6.0a0 is a
#   pre-release of 2.6 that predates that kwarg, so the gate says yes and the
#   DataLoader says no.
# new: Qwen3.5 (`model_type: qwen3_5`) is only known to transformers 5.x, which
#   forces a real torch release. That replaces the NGC build, so the NGC-built
#   flash-attn stops importing and attention falls back to sdpa — correct, just
#   slower. Recorded in the run registry so the 4B/9B comparison carries the
#   caveat.
# The NGC image sets pypi.ngc.nvidia.com as an extra index in /etc/pip.conf and
# that hostname does not resolve here, so every install burns its retry budget on
# a dead index. An empty PIP_EXTRA_INDEX_URL does NOT override the config file —
# pip reads empty as unset and falls back to it — so the index has to be given on
# the command line, where CLI beats config.
PIP_ARGS="--index-url https://pypi.org/simple --no-cache-dir -q --retries 5 --timeout 60"

DEPS="${DEPS:-ngc}"
if [ "$DEPS" = "new" ]; then
  PKGS='torch==2.9.1 transformers==5.15.0 trl==1.10.0 peft==0.20.0'
else
  PKGS='transformers==4.57.1 trl==0.24.0 peft==0.17.1'
fi

# DNS on these nodes is intermittently unresolvable, and pip exhausting its
# retries leaves a PARTIAL install rather than failing: transformers ends up
# importable but its lazy submodules are not, which surfaces later as
# "Could not import module 'TrainingArguments'" — an error that points nowhere
# near the real cause. So verify by importing, and reinstall if that fails.
install_deps() {
  # shellcheck disable=SC2086
  python -m pip install $PIP_ARGS \
    $PKGS datasets accelerate wandb huggingface_hub
}
verify_deps() {
  python - <<'PY'
import sys
try:
    import torch, transformers, trl, peft, datasets, accelerate  # noqa: F401
    from transformers import TrainingArguments, AutoModelForCausalLM  # noqa: F401
    from trl import SFTTrainer, SFTConfig  # noqa: F401
    from peft import LoraConfig  # noqa: F401
    print(f"[entry] deps ok: torch={torch.__version__} "
          f"transformers={transformers.__version__} trl={trl.__version__} peft={peft.__version__}")
except Exception as e:
    print(f"[entry] deps BROKEN: {type(e).__name__}: {e}", file=sys.stderr)
    sys.exit(1)
PY
}

echo "[entry] installing deps (profile=$DEPS)"
for attempt in 1 2 3; do
  install_deps || true
  if verify_deps; then break; fi
  echo "[entry] install attempt $attempt left the env broken; retrying"
  [ "$attempt" = 3 ] && { echo "[entry] FATAL: deps unusable after 3 attempts"; exit 1; }
  sleep 20
done

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
