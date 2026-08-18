#!/usr/bin/env bash
# Score a generated spec directory on the two bug-finding benchmarks.
#
#   scripts/score_benchmarks.sh gen3-4b-final            # pulls from the ckpt repo
#   scripts/score_benchmarks.sh --local path/to/specs    # scores a directory you have
#
# The benchmarks live on the `baseline1-general-model-rule-check` branch, not this
# one, and are NOT merged here on purpose: that branch is 2038 files of in-flight
# work by someone else, and mixing it into the training branch would make both
# harder to review. This script drives them from a detached worktree instead, so
# each branch stays as its author left it.
#
# Two benchmarks, two very different requirements:
#
#   rule_check_8bugs   textual. Asks whether a declared output is left
#                      unconstrained. No Verus, runs anywhere. Its 8 findings are
#                      eac5/rel0 items.
#   verus_rmm          asks Verus to accept a contradiction proof or reject an
#                      obligation. Needs a verus binary AND a rustup toolchain.
#
# A word on what a flag here means. The rule check rewards leaving an output
# unconstrained, and gold leaves it unconstrained because that IS the gap being
# detected. So for a checkpoint trained on gold for that command, a flag is not
# evidence: the eac5 training example and the eac5 test are the same
# (input, target) pair. Five of the eight are in every sft2-* training set at all
# six versions. Only sft3-* (dataset_bench) can be scored here honestly.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BENCH_BRANCH="${BENCH_BRANCH:-origin/baseline1-general-model-rule-check}"
WORK="${BENCH_WORKTREE:-${TMPDIR:-/tmp}/spec-check-bench}"

LOCAL_DIR=""
if [[ "${1:-}" == "--local" ]]; then
  # Absolute, immediately. Both scorers are invoked after `cd` into another
  # directory, so a relative path silently resolves somewhere else, finds no
  # files, and reports a clean 0/4 instead of failing. That cost a wrong
  # conclusion once already.
  LOCAL_DIR="$(cd "${2:?--local needs a path}" && pwd)"
  shift 2
fi
NAME="${1:-}"
[[ -n "$LOCAL_DIR" || -n "$NAME" ]] || {
  echo "usage: $0 <gen-name>   |   $0 --local <dir>" >&2; exit 1; }

# --- benchmark worktree ------------------------------------------------------
if [[ ! -d "$WORK/benchmark" ]]; then
  echo "==> checking out $BENCH_BRANCH at $WORK"
  git -C "$REPO_ROOT" fetch -q origin
  git -C "$REPO_ROOT" worktree add -q --detach "$WORK" "$BENCH_BRANCH"
fi

# --- the specs to score ------------------------------------------------------
if [[ -z "$LOCAL_DIR" ]]; then
  LOCAL_DIR="$WORK/../gen-$NAME"
  if [[ ! -d "$LOCAL_DIR" ]]; then
    echo "==> pulling gen/$NAME.tgz from the checkpoint repo"
    mkdir -p "$LOCAL_DIR"
    python3 - "$NAME" "$LOCAL_DIR" <<'PY'
import os, sys, tarfile, io, urllib.request
name, dest = sys.argv[1], sys.argv[2]
env = os.environ.get("HF_TOKEN")
if not env:  # .env is gitignored and is where the token lives locally
    for line in open(os.path.join(os.path.dirname(__file__), "..", ".env")):
        line = line.strip()
        if line.startswith("HF_TOKEN="):
            env = line.split("=", 1)[1].strip().strip('"').strip("'")
repo = os.environ.get("HF_CKPT_REPO", "jisenli/spec-check-ckpt")
url = f"https://huggingface.co/{repo}/resolve/main/gen/{name}.tgz"
req = urllib.request.Request(url, headers={"Authorization": f"Bearer {env}"})
with tarfile.open(fileobj=io.BytesIO(urllib.request.urlopen(req).read())) as t:
    t.extractall(dest)
print(f"  extracted to {dest}")
PY
  fi
  # Unwrap the tarball's own top directory. This must happen on the cached path
  # too, not only right after extracting: on a second run the download is skipped
  # and LOCAL_DIR would still point at the wrapper, so every version directory
  # comes up empty and the scorer reports a cheerful 0/0.
  if [[ ! -d "$LOCAL_DIR/eac5" && ! -d "$LOCAL_DIR/alp14" ]]; then
    inner="$(find "$LOCAL_DIR" -mindepth 1 -maxdepth 1 -type d | head -1)"
    [[ -n "$inner" ]] && LOCAL_DIR="$inner"
  fi
fi
LOCAL_DIR="$(cd "$LOCAL_DIR" && pwd)"
NRS="$(find "$LOCAL_DIR" -name '*.rs' | wc -l | tr -d ' ')"
echo "==> scoring $LOCAL_DIR"
echo "    $NRS spec files"
# An empty input scores 0/4 with everything inconclusive, which reads exactly
# like a real bad result. Refuse instead.
[[ "$NRS" -gt 0 ]] || { echo "no .rs files under $LOCAL_DIR -- refusing to report a score" >&2; exit 1; }

# --- 1. rule check (no Verus) ------------------------------------------------
echo
echo "=== rule_check_8bugs ==="
PRED="$WORK/benchmark/rule_check_8bugs/predictions/_scoring_tmp"
rm -rf "$PRED"; mkdir -p "$PRED"
cp -R "$LOCAL_DIR"/. "$PRED"/
# The scorer is run from its own directory: its defaults are relative to it.
( cd "$WORK/benchmark/rule_check_8bugs" && python3 score.py --model _scoring_tmp ) || true
rm -rf "$PRED"

# --- 2. verus_rmm (needs Verus) ----------------------------------------------
echo
echo "=== verus_rmm ==="
# run_bench.py hardcodes ROOT/training/verus-x86-linux/verus. Rather than patch
# someone else's file, symlink whatever binary we have into that path.
if [[ -n "${VERUS_BIN:-}" && -x "${VERUS_BIN}" ]]; then
  mkdir -p "$WORK/training/verus-x86-linux"
  ln -sf "$VERUS_BIN" "$WORK/training/verus-x86-linux/verus"
  for v in eac5 rel0 alp14; do
    [[ -d "$LOCAL_DIR/$v" ]] || continue
    nv="$(find "$LOCAL_DIR/$v" -name '*.rs' | wc -l | tr -d ' ')"
    echo "--- $v  ($nv files)"
    [[ "$nv" -gt 0 ]] || { echo "    empty, skipping"; continue; }
    # --version selects BOTH the ground truth and the preamble. Without it every
    # version is scored against eac5's, which silently mislabels: rel0 specs read
    # against eac5 ground truth report items that do not exist in rel0 and go
    # inconclusive on symbols eac5's preamble lacks.
    ( cd "$WORK" && python3 benchmark/verus_rmm/run_bench.py \
        --version "$v" \
        --gen-dir "$LOCAL_DIR/$v" --gen-pattern '{cmd}.rs' \
        --label "$(basename "$LOCAL_DIR")-$v" ) || true
  done
else
  echo "  skipped: set VERUS_BIN to a verus binary for this platform."
  echo "  macOS arm64 build of the version this project uses:"
  echo "    https://github.com/verus-lang/verus/releases/download/release/0.2026.04.12.f1166c4/verus-0.2026.04.12.f1166c4-arm64-macos.zip"
  echo "  It also needs a rustup toolchain (1.94.0); RUSTUP_HOME/CARGO_HOME may"
  echo "  point anywhere, so it need not be installed into your home directory."
fi
