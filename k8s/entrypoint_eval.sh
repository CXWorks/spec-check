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
SAMPLES="${SAMPLES:-0}"          # >0 turns on best-of-k on top of the greedy sample
TEMPERATURE="${TEMPERATURE:-0.8}"
JOBS="${JOBS:-8}"
OUT_TAG="${OUT_TAG:-}"           # suffix so a best-of-k run cannot overwrite a greedy one
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
  # Installing a real torch over the NGC build leaves the NGC torchvision behind,
  # compiled against the old one. It then fails with "operator torchvision::nms
  # does not exist" the moment transformers touches its import chain — surfacing
  # as an unrelated-looking "Could not import module 'BloomPreTrainedModel'". We
  # train text models; a broken torchvision is worse than none.
  python -m pip uninstall -y -q torchvision 2>/dev/null || true
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
  curl -sSL --retry 5 --retry-all-errors --retry-delay 10 -o verus.zip \
    "https://github.com/verus-lang/verus/releases/download/release/${VERUS_VER}/verus-${VERUS_VER}-x86-linux.zip"
  command -v unzip >/dev/null || (apt-get update -qq && apt-get install -y -qq unzip) >/dev/null 2>&1
  rm -rf verus && unzip -q verus.zip -d verus
fi
"$VERUS_BIN" --version | head -1

# Retry the fetch in-process rather than letting the pod die on it.
#
# DNS here fails intermittently, and huggingface_hub turns that into
# LocalEntryNotFoundError rather than retrying long enough to ride it out. Losing
# the pod to that is much worse than it sounds: these PVCs are local-path, so the
# replacement pod is pinned to the SAME node by its volume and hits the same
# blip, walking straight through backoffLimit. bok-0 and bok-1 burned three
# attempts each this way while DNS on the node was fine seconds later.
fetch_data() {
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
}

for a in 1 2 3 4 5 6; do
  fetch_data && break
  [ "$a" = 6 ] && { echo "[eval] FATAL: cannot fetch data after 6 attempts"; exit 1; }
  echo "[eval] fetch attempt $a failed (likely transient DNS); retrying in 30s"
  sleep 30
done

cp -r /work/code/* /work/repo/ 2>/dev/null || true
ls scripts/eval_checkpoint.py prompt_engineering/dataset_loader.py >/dev/null

SAMPLE_ARGS=""
[ "$SAMPLES" -gt 0 ] && SAMPLE_ARGS="--samples $SAMPLES --temperature $TEMPERATURE"

mkdir -p /work/eval
for RUN in $RUN_IDS; do
  for CK in $CKPTS; do
    NAME="${RUN}-${CK}${OUT_TAG}"
    OUT="/work/eval/${NAME}.json"
    echo "[eval] ===== $RUN / $CK ${SAMPLE_ARGS} ====="
    # Retried for the same reason as the data fetch: the checkpoint download can
    # hit the same DNS blip, and `continue` alone would silently drop a run from
    # the comparison — the worst failure mode here, because the summary table
    # would still look complete.
    ok=""
    for a in 1 2 3; do
      # shellcheck disable=SC2086
      python scripts/eval_checkpoint.py \
        --base "$BASE_MODEL" \
        --adapter "${HF_CKPT_REPO}" --subfolder "${RUN}/${CK}" \
        --jobs "$JOBS" $SAMPLE_ARGS \
        --out "$OUT" && { ok=1; break; }
      echo "[eval] attempt $a failed for $RUN/$CK; retrying in 30s"
      sleep 30
    done
    [ -n "$ok" ] || { echo "[eval] FAILED $RUN/$CK after 3 attempts"; continue; }
    python - <<PY
import json, os
from huggingface_hub import HfApi
HfApi(token=os.environ["HF_TOKEN"]).upload_file(
    path_or_fileobj="$OUT", path_in_repo="eval/${NAME}.json",
    repo_id=os.environ["HF_CKPT_REPO"], repo_type="model")
print("[eval] uploaded eval/${NAME}.json")
PY
  done
done
echo "[eval] done"
