#!/usr/bin/env bash
# run_opus5_eval.sh -- Baseline-2 rerun with claude-opus-5 (effort=high) via `claude -p`.
#
# Every stage is resumable and the driver records the last completed stage, so
# it can be re-invoked after any interruption (usage limit, crash, reboot) and
# will pick up where it stopped.  Quota suspension itself is handled inside
# ClaudeCLIModel (suspend -> probe -> resume), not here.
#
#   ./run_opus5_eval.sh            # run all remaining stages
#   ./run_opus5_eval.sh 3          # force-restart from stage 3
set -uo pipefail
cd "$(dirname "$0")"

export VERUS_BIN=${VERUS_BIN:-/mnt/sdc/xiang/spec-check/training/verus-x86-linux/verus}
export VERUSFMT_BIN=${VERUSFMT_BIN:-/mnt/sdc/xiang/spec-check/training/verusfmt/target/release/verusfmt}
RAG=${RAG:-/mnt/sdc/xiang/spec-check/rag/index.json}
RAW=${RAW:-/mnt/sdc/xiang/spec-check/work/scope_run/alp14_raw.txt}

KEY=v3_opus5
BASE=results/ab_test/$KEY
OUT=$BASE/alp14
MODEL=claude-opus-5
EFFORT=high
STAGEFILE=$BASE/STAGE
LOGDIR=$BASE/logs

mkdir -p "$BASE" "$LOGDIR"
LAST=$(cat "$STAGEFILE" 2>/dev/null || echo 0)
START=${1:-$((LAST + 1))}

say() { echo "[$(date '+%F %T')] $*" | tee -a "$LOGDIR/driver.log"; }
done_stage() { echo "$1" > "$STAGEFILE"; say "stage $1 complete"; }

run_stage() {
  local n=$1; shift
  [ "$n" -lt "$START" ] && { say "stage $n: skipped (already done)"; return 0; }
  say "stage $n: starting -- $*"
  if "$@" 2>&1 | tee -a "$LOGDIR/stage$n.log"; then
    done_stage "$n"
  else
    say "stage $n: FAILED (exit ${PIPESTATUS[0]}); rerun this script to retry"
    exit 1
  fi
}

# ---- stage 1: generation (98 commands x 5 samples) -------------------------
run_stage 1 python3 prompt_engineering/prompt_engineering_v3.py \
    --split test --limit 98 --n-samples 5 \
    --rag-index "$RAG" --rag-top-k 3 --save-results --resume \
    --backend cli --model "$MODEL" --effort "$EFFORT" --variant-key "$KEY"

# ---- stage 2: snapshot pre-repair artifacts --------------------------------
if [ "$START" -le 2 ]; then
  say "stage 2: snapshotting pre-repair artifacts"
  rm -rf "${OUT}_prerepair"
  cp -a "$OUT" "${OUT}_prerepair"
  done_stage 2
fi

# ---- stage 3: Verus check (pre-repair) -------------------------------------
run_stage 3 python3 prompt_engineering/verify_generated_verus.py \
    --results-root "$OUT" --specs-dir training-dataset/specs/alp14 --verus "$VERUS_BIN"

# ---- stage 4: Verus-feedback repair loop -----------------------------------
run_stage 4 python3 repair_loop_verus_claude.py \
    --verus-summary "$BASE/alp14_verus_check_summary.json" \
    --results-root "$OUT" --specs-dir training-dataset/specs/alp14 \
    --verus "$VERUS_BIN" --backend cli --model "$MODEL" --effort "$EFFORT" \
    --max-retries 10 --resume

# ---- stage 5: independent post-repair recheck + CodeBLEU -------------------
# Independent recheck guards against the repair loop's self-report disagreeing
# with reality (the Iteration-7 stale-cache incident surfaced exactly that way).
run_stage 5 bash -c "
  python3 prompt_engineering/verify_generated_verus.py \
      --results-root '$OUT' --specs-dir training-dataset/specs/alp14 --verus '$VERUS_BIN' \
      --json-out '$BASE/alp14_verus_check_summary_recheck.json' &&
  python3 tools/score_opus5.py --base '$BASE'
"

# ---- stage 6: rule-based checks --------------------------------------------
run_stage 6 bash -c "
  python3 training/scope_rule_check_ourcode.py --raw-file '$RAW' --gen-dir '$OUT' ;
  python3 training/footprint_check_normalized.py --raw-file '$RAW' --gen-dir '$OUT'
"

# ---- stage 7: Z3 inconsistency sweep ---------------------------------------
run_stage 7 python3 training/inconsistency_analysis_model.py \
    --results-root "$OUT" --specs-dir training-dataset/specs/alp14 \
    --verus "$VERUS_BIN" --json-out "$BASE/alp14_inconsistency_sweep.json"

say "ALL STAGES COMPLETE"
