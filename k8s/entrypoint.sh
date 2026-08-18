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
SEED="${SEED:-42}"
DATA_REPO="${DATA_REPO:-jisenli/spec-check-data}"
# Which built dataset to train on. There is no PROMPT_VARIANT here on purpose:
# build_dataset.py has already baked the system prompt into every example, so
# train.py never renders one and a knob here would configure nothing. The
# variant is a property OF the dataset, read back from its splits.json below —
# and the eval job must be given the matching --prompt-variant, or it renders a
# different prompt than the checkpoint was trained on (RESULTS_V3.md Iteration 7).
DATASET_DIR="${DATASET_DIR:-dataset_clean}"
EXTRA_ARGS="${EXTRA_ARGS:-}"

echo "[entry] run=$RUN_ID model=$BASE_MODEL precision=$PRECISION method=$METHOD seed=$SEED"
echo "[entry] dataset=$DATASET_DIR"

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
# The NGC image leaves pypi.ngc.nvidia.com as an extra index, and that hostname
# does not resolve from either cluster, so every package burns five retries on a
# dead index before falling through to pypi.org.
#
# The file is /root/.config/pip/pip.conf (and /root/.pip/pip.conf) — USER-level
# config — NOT /etc/pip.conf as this comment previously claimed. Both /etc/pip.conf
# and /usr/pip.conf were checked on a live pod and have `extra-index-url =` empty,
# which is why overriding only --index-url never silenced it: --index-url replaces
# `index-url`, and leaves `extra-index-url` untouched.
#
# Overriding on the command line does NOT remove it, which is what the previous
# version of this comment claimed. `extra-index-url` is an append option: a CLI
# --extra-index-url is ADDED to the config's list, so the dead host stayed in it
# and every nvidia-* package still burned 5 retries x 60s resolving a name that
# does not exist. Setting PIP_EXTRA_INDEX_URL empty does not work either: pip
# reads empty as unset and falls back to the config.
#
# Ignoring the config file outright is the override that actually removes it.
export PIP_CONFIG_FILE=/dev/null
PIP_ARGS="--index-url https://pypi.org/simple --extra-index-url https://pypi.org/simple --no-cache-dir -q --retries 5 --timeout 60"

DEPS="${DEPS:-ngc}"
if [ "$DEPS" = "new" ]; then
  PKGS='torch==2.9.1 transformers==5.15.0 trl==0.24.0 peft==0.20.0'
  # Installing a real torch over the NGC build leaves the NGC torchvision behind,
  # compiled against the old one. It then fails with "operator torchvision::nms
  # does not exist" the moment transformers touches its import chain — surfacing
  # as an unrelated-looking "Could not import module 'BloomPreTrainedModel'". We
  # train text models; a broken torchvision is worse than none.
  python -m pip uninstall -y -q torchvision 2>/dev/null || true
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
# Retried for the same reason as the pip loop above: DNS here fails
# intermittently and huggingface_hub raises LocalEntryNotFoundError rather than
# waiting it out. Letting the pod die costs more than it looks — local-path PVCs
# pin the replacement pod to the same node, so it meets the same blip and walks
# through backoffLimit without ever moving.
fetch_data() {
echo "[entry] fetching dataset from $DATA_REPO"
python - <<PY
from huggingface_hub import snapshot_download
import os, shutil
p = snapshot_download(repo_id="${DATA_REPO}", repo_type="dataset",
                      token=os.environ.get("HF_TOKEN"),
                      allow_patterns=["${DATASET_DIR}/*"])
src = os.path.join(p, "${DATASET_DIR}")
assert os.path.isdir(src), (
    "${DATASET_DIR}/ is not in the data repo - upload it before training on it")
shutil.copytree(src, "/work/data/${DATASET_DIR}", dirs_exist_ok=True)
import json
sp = os.path.join(src, "splits.json")
if os.path.exists(sp):
    d = json.load(open(sp))
    print(f"[entry] dataset ready: {len(d['command_test'])} held out, "
          f"prompt {d.get('prompt_variant', '?')}")
else:
    print("[entry] dataset ready")
PY
}
for a in 1 2 3 4 5 6; do
  fetch_data && break
  [ "$a" = 6 ] && { echo "[entry] FATAL: cannot fetch dataset after 6 attempts"; exit 1; }
  echo "[entry] fetch attempt $a failed (likely transient DNS); retrying in 30s"
  sleep 30
done
wc -l /work/data/$DATASET_DIR/*.jsonl

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
  --train /work/data/$DATASET_DIR/train.jsonl \
  --val   /work/data/$DATASET_DIR/val.jsonl \
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
  --seed "$SEED" \
  --push $EXTRA_ARGS

echo "[entry] done: $RUN_ID"
