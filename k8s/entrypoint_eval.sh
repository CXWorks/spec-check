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
MODE="${MODE:-score}"            # score | repair | gen
# gen: write one <version>/<command>.rs per command for the rule-check benchmark
# (benchmark/rule_check_8bugs/score.py). Its eight findings are eac5/rel0 items,
# so this mode needs those versions' section text, not alp14's, and it needs no
# Verus at all -- the dangling-output check is textual.
GEN_VERSIONS="${GEN_VERSIONS:-eac5 rel0}"
# >0 makes gen mode compile each spec and feed the error back. That needs verus
# and rustup after all, so the skips below are conditional on it being 0.
REPAIR_ROUNDS="${REPAIR_ROUNDS:-0}"
WITH_PREAMBLE="${WITH_PREAMBLE:-0}"  # 1 restores the preamble that training used
# tail = the 200-line window training used and every published number was
# produced with. selected = declarations named in the command's own section;
# the tail hides 51% of the API gold uses on alp14.
PREAMBLE_MODE="${PREAMBLE_MODE:-tail}"
FRAME_HINT="${FRAME_HINT:-0}"        # 1 demands frame conditions explicitly
ROUNDS="${ROUNDS:-2}"            # repair mode only
SAMPLES="${SAMPLES:-0}"          # >0 turns on best-of-k on top of the greedy sample
TEMPERATURE="${TEMPERATURE:-0.8}"
JOBS="${JOBS:-8}"
OUT_TAG="${OUT_TAG:-}"           # suffix so a best-of-k run cannot overwrite a greedy one
DATA_REPO="${DATA_REPO:-jisenli/spec-check-data}"
# Must match the checkpoint's training dataset and prompt, both recorded in that
# dataset's splits.json. Unlike training, eval RENDERS the prompt, so a mismatch
# here silently scores a checkpoint with a prompt it never saw — the failure
# RESULTS_V3.md Iteration 7 spent an iteration on. DATASET_DIR also selects the
# held-out list: dataset_bench holds out 49 commands where dataset_clean holds 40.
DATASET_DIR="${DATASET_DIR:-dataset_clean}"
PROMPT_VARIANT="${PROMPT_VARIANT:-v3}"
export SPEC_CHECK_DATASET_DIR="$DATASET_DIR"
VERUS_VER="0.2026.04.12.f1166c4"  # the version the project's history used

echo "[eval] runs=$RUN_IDS base=$BASE_MODEL ckpts=$CKPTS"
echo "[eval] dataset=$DATASET_DIR prompt=$PROMPT_VARIANT"

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
if [ "$MODE" = "gen" ] && [ "$REPAIR_ROUNDS" = "0" ]; then
  # rustup exists only to satisfy verus, which gen mode invokes only when
  # repairing.
  echo "[eval] mode=gen without repair: skipping rustup"
elif [ ! -x "$CARGO_HOME/bin/rustc" ]; then
  echo "[eval] installing rustup"
  mkdir -p "$RUSTUP_HOME" "$CARGO_HOME"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --no-modify-path --default-toolchain 1.94.0-x86_64-unknown-linux-gnu >/dev/null
fi

export VERUS_BIN=/work/tools/verus/verus-x86-linux/verus
if [ "$MODE" = "gen" ] && [ "$REPAIR_ROUNDS" = "0" ]; then
  echo "[eval] mode=gen without repair: skipping verus (the check is textual)"
elif [ ! -x "$VERUS_BIN" ]; then
  echo "[eval] installing verus $VERUS_VER"
  mkdir -p /work/tools && cd /work/tools
  curl -sSL --retry 5 --retry-all-errors --retry-delay 10 -o verus.zip \
    "https://github.com/verus-lang/verus/releases/download/release/${VERUS_VER}/verus-${VERUS_VER}-x86-linux.zip"
  command -v unzip >/dev/null || (apt-get update -qq && apt-get install -y -qq unzip) >/dev/null 2>&1
  rm -rf verus && unzip -q verus.zip -d verus
fi
{ [ "$MODE" = "gen" ] && [ "$REPAIR_ROUNDS" = "0" ]; } || "$VERUS_BIN" --version | head -1

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
src = os.path.join(p, "${DATASET_DIR}")
assert os.path.isdir(src), (
    "${DATASET_DIR}/ is not in the data repo - upload it before scoring against it")
shutil.copytree(src, "training-dataset/${DATASET_DIR}", dirs_exist_ok=True)
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
print(f"[eval] data ready ({n} alp14 sections)")

# gen mode scores eac5/rel0, whose sections are separate archives in the repo.
for v in ("${GEN_VERSIONS}".split() if "${MODE}" == "gen" else []):
    os.makedirs(f"training-dataset/specs/{v}", exist_ok=True)
    shutil.copy(os.path.join(p, f"specs/{v}/preamble.rs"), f"training-dataset/specs/{v}/")
    # Gold too, not just sections: dataset_loader.load_version SKIPS any command
    # whose gold spec is missing, so sections alone load 0 commands and the
    # generation emits nothing at all. Generation does not read gold -- the loader
    # simply refuses to build a sample without it.
    with tarfile.open(os.path.join(p, f"specs/{v}_gold.tgz")) as t:
        t.extractall(f"training-dataset/specs/{v}")
    with tarfile.open(os.path.join(p, f"sections/{v}.tgz")) as t:
        t.extractall("training-dataset")
    m = len([f for f in os.listdir(f"training-dataset/sections/{v}")
             if f.endswith("_command.txt")])
    g = len([f for f in os.listdir(f"training-dataset/specs/{v}")
             if f.endswith("_spec.rs")])
    assert m > 0 and g > 0, (
        f"{v}: {m} sections, {g} gold specs - the loader needs BOTH and would "
        "otherwise load 0 commands")
    print(f"[eval] {v} ready ({m} sections, {g} gold)")
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

# A run that fails after its retries must FAIL the Job. `continue` alone lets the
# loop finish and the script exit 0, so k8s reports Complete with no artifact --
# the same silent-success failure that once recorded three seed replicates as
# done with nothing behind them. gen3-4b reproduced it exactly.
FAILED_RUNS=""

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
    if [ "$MODE" = "gen" ]; then
      # Output is a directory of .rs files rather than a JSON, so this branch has
      # its own invocation and its own upload instead of sharing the score path.
      GDIR="/work/eval/${NAME}"
      # The PVC is per-Job but outlives the POD, and backoffLimit is 20: every
      # retry of a failed pod remounts this same directory, as does resubmitting
      # the Job under the same name without deleting the PVC. The tar below takes
      # the whole directory, so a partial run's .rs files ship inside the next
      # attempt's artifact and are scored as if that attempt had produced them.
      #
      # No artifact is known to have been affected. This guard was added after a
      # false alarm -- gen/sft3-2-final.tgz has 82 files and looked contaminated,
      # but its Job generated all 82; the GEN_VERSIONS="eac5 rel0" that produced
      # them had been split on the space by the command used to inspect it. The
      # guard is kept because the retry path above is real, not because that was.
      rm -rf "$GDIR"
      GEN_ARGS="--versions $GEN_VERSIONS --out-dir $GDIR --repair-rounds $REPAIR_ROUNDS"
      [ "$WITH_PREAMBLE" = "1" ] && GEN_ARGS="$GEN_ARGS --with-preamble --preamble-mode $PREAMBLE_MODE"
      ok=""
      for a in 1 2 3; do
        # shellcheck disable=SC2086
        python scripts/gen_specs.py \
          --base "$BASE_MODEL" \
          --adapter "${HF_CKPT_REPO}" --subfolder "${RUN}/${CK}" \
          --prompt-variant "$PROMPT_VARIANT" $GEN_ARGS && { ok=1; break; }
        echo "[eval] attempt $a failed for $RUN/$CK; retrying in 30s"
        sleep 30
      done
      [ -n "$ok" ] || { echo "[eval] FAILED $RUN/$CK after 3 attempts"
                        FAILED_RUNS="$FAILED_RUNS $RUN/$CK"; continue; }
      echo "[eval] generated $(find "$GDIR" -name '*.rs' | wc -l) spec files"
      # Per-version, so the log shows which versions are actually in the artifact
      # rather than only which ones were requested. Those two disagreed once.
      for gv in $GEN_VERSIONS; do
        echo "[eval]   $gv: $(find "$GDIR/$gv" -name '*.rs' 2>/dev/null | wc -l) files"
      done
      tar czf "/work/eval/${NAME}.tgz" -C /work/eval "${NAME}"
      python - <<UPLOAD_EOF
import os
from huggingface_hub import HfApi
HfApi(token=os.environ["HF_TOKEN"]).upload_file(
    path_or_fileobj="/work/eval/${NAME}.tgz", path_in_repo="gen/${NAME}.tgz",
    repo_id=os.environ["HF_CKPT_REPO"], repo_type="model")
print("[eval] uploaded gen/${NAME}.tgz")
UPLOAD_EOF
      continue
    fi
    if [ "$MODE" = "repair" ]; then
      SCRIPT="scripts/repair_eval.py"; MODE_ARGS="--rounds $ROUNDS"
    else
      SCRIPT="scripts/eval_checkpoint.py"; MODE_ARGS="$SAMPLE_ARGS"
      [ "$WITH_PREAMBLE" = "1" ] && MODE_ARGS="$MODE_ARGS --with-preamble"
      [ "$FRAME_HINT" = "1" ] && MODE_ARGS="$MODE_ARGS --frame-hint"
    fi
    ok=""
    for a in 1 2 3; do
      # shellcheck disable=SC2086
      python "$SCRIPT" \
        --base "$BASE_MODEL" \
        --adapter "${HF_CKPT_REPO}" --subfolder "${RUN}/${CK}" \
        --jobs "$JOBS" --prompt-variant "$PROMPT_VARIANT" $MODE_ARGS \
        --out "$OUT" && { ok=1; break; }
      echo "[eval] attempt $a failed for $RUN/$CK; retrying in 30s"
      sleep 30
    done
    [ -n "$ok" ] || { echo "[eval] FAILED $RUN/$CK after 3 attempts"
                      FAILED_RUNS="$FAILED_RUNS $RUN/$CK"; continue; }
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

if [ -n "$FAILED_RUNS" ]; then
  echo "[eval] FATAL: no artifact produced for:$FAILED_RUNS"
  exit 1
fi
echo "[eval] done"
