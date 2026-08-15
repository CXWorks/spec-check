#!/bin/bash
# Eval Job entrypoint: score checkpoints on the 40 held-out commands.
#
# Runs as a Job rather than inside the long-lived pod because DNS on individual
# nodes is intermittently unresolvable, and a Job that lands somewhere bad can
# simply be resubmitted onto a different node.
set -euo pipefail

: "${RUN_IDS:?RUN_IDS is required, e.g. 'sft2-0 sft2-1'}"
: "${BASE_MODEL:?BASE_MODEL is required}"
DEPS="${DEPS:-new}"
CKPTS="${CKPTS:-final}"          # space-separated: final checkpoint-41 ...
DATA_REPO="${DATA_REPO:-jisenli/spec-check-data}"
VERUS_VER="0.2026.04.12.f1166c4"  # the version the project's history used

echo "[eval] runs=$RUN_IDS base=$BASE_MODEL ckpts=$CKPTS"

# The NGC image sets pypi.ngc.nvidia.com as an extra index in /etc/pip.conf and
# that hostname does not resolve here, so every install burns its retry budget on
# a dead index. An empty PIP_EXTRA_INDEX_URL does NOT override the config file —
# pip reads empty as unset and falls back to it — so the index has to be given on
# the command line, where CLI beats config.
PIP_ARGS="--index-url https://pypi.org/simple --no-cache-dir -q --retries 5 --timeout 60"

if [ "$DEPS" = "new" ]; then
  PKGS='torch==2.9.1 transformers==5.15.0 peft==0.20.0'
else
  PKGS='transformers==4.57.1 peft==0.17.1'
fi

verify_deps() {
  python - <<'PY'
import sys
try:
    import torch, transformers, peft  # noqa: F401
    from transformers import AutoModelForCausalLM, AutoTokenizer  # noqa: F401
    from peft import PeftModel  # noqa: F401
    print(f"[eval] deps ok: torch={torch.__version__} tf={transformers.__version__} peft={peft.__version__}")
except Exception as e:
    print(f"[eval] deps BROKEN: {type(e).__name__}: {e}", file=sys.stderr); sys.exit(1)
PY
}

echo "[eval] installing deps"
for a in 1 2 3; do
  # shellcheck disable=SC2086
  python -m pip install $PIP_ARGS \
    $PKGS datasets accelerate huggingface_hub codebleu || true
  verify_deps && break
  [ "$a" = 3 ] && { echo "[eval] FATAL: deps unusable"; exit 1; }
  echo "[eval] retry $a"; sleep 20
done

# Verus needs a rustup toolchain, and the preamble needs read_preamble()'s
# struct -> pub struct rewrite; both are prerequisites, not optional extras.
export RUSTUP_HOME=/work/rust/rustup CARGO_HOME=/work/rust/cargo
export PATH=$CARGO_HOME/bin:$PATH
if [ ! -x "$CARGO_HOME/bin/rustc" ]; then
  echo "[eval] installing rustup"
  mkdir -p "$RUSTUP_HOME" "$CARGO_HOME"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --no-modify-path --default-toolchain 1.94.0-x86_64-unknown-linux-gnu >/dev/null
fi

export VERUS_BIN=/work/tools/verus/verus-x86-linux/verus
if [ ! -x "$VERUS_BIN" ]; then
  echo "[eval] installing verus $VERUS_VER"
  mkdir -p /work/tools && cd /work/tools
  curl -sSL -o verus.zip \
    "https://github.com/verus-lang/verus/releases/download/release/${VERUS_VER}/verus-${VERUS_VER}-x86-linux.zip"
  command -v unzip >/dev/null || (apt-get update -qq && apt-get install -y -qq unzip) >/dev/null 2>&1
  rm -rf verus && unzip -q verus.zip -d verus
fi
"$VERUS_BIN" --version | head -1

echo "[eval] fetching data + code"
mkdir -p /work/repo && cd /work/repo
python - <<PY
from huggingface_hub import snapshot_download
import os, shutil, tarfile
p = snapshot_download(repo_id="${DATA_REPO}", repo_type="dataset",
                      token=os.environ.get("HF_TOKEN"))
os.makedirs("training-dataset", exist_ok=True)
shutil.copytree(os.path.join(p, "dataset_clean"), "training-dataset/dataset_clean",
                dirs_exist_ok=True)
os.makedirs("training-dataset/specs/alp14", exist_ok=True)
shutil.copy(os.path.join(p, "specs/alp14/preamble.rs"), "training-dataset/specs/alp14/")
with tarfile.open(os.path.join(p, "specs/alp14_gold.tgz")) as t:
    t.extractall("training-dataset/specs/alp14")
# The section text is the model's INPUT, not an extra: without it load_version
# finds nothing and the eval silently scores 0 commands.
with tarfile.open(os.path.join(p, "sections/alp14.tgz")) as t:
    t.extractall("training-dataset")
n = len([f for f in os.listdir("training-dataset/sections/alp14") if f.endswith("_command.txt")])
assert n > 0, "no section files - eval would score nothing"
print(f"[eval] data ready ({n} sections)")
PY
cp -r /work/code/* /work/repo/ 2>/dev/null || true
ls scripts/eval_checkpoint.py prompt_engineering/dataset_loader.py >/dev/null

mkdir -p /work/eval
for RUN in $RUN_IDS; do
  for CK in $CKPTS; do
    OUT="/work/eval/${RUN}-${CK}.json"
    echo "[eval] ===== $RUN / $CK ====="
    python scripts/eval_checkpoint.py \
      --base "$BASE_MODEL" \
      --adapter "${HF_CKPT_REPO}" --subfolder "${RUN}/${CK}" \
      --out "$OUT" || { echo "[eval] FAILED $RUN/$CK"; continue; }
    python - <<PY
import json, os
from huggingface_hub import HfApi
HfApi(token=os.environ["HF_TOKEN"]).upload_file(
    path_or_fileobj="$OUT", path_in_repo="eval/${RUN}-${CK}.json",
    repo_id=os.environ["HF_CKPT_REPO"], repo_type="model")
print("[eval] uploaded eval/${RUN}-${CK}.json")
PY
  done
done
echo "[eval] done"
