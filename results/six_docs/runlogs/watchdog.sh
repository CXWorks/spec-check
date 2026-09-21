#!/usr/bin/env bash
# Wait for the running batch, then queue SCMI and SBI (12 more runs).
#
# Written to a FILE and run as "bash watchdog.sh" on purpose. The previous
# waiter was a `bash -c` whose own command line contained "gen_specs.py", so
# its pgrep matched itself, the loop never ended, and it sat for 2d23h. This
# script's own cmdline is just its path, so it cannot match the pattern.
set -u
R=/shared/jisenli2/rulecheck
LOG=$R/watchdog.log
say() { echo "[$(date '+%m-%d %H:%M:%S')] $*" >> "$LOG"; }

say "watchdog up (pid $$), waiting for the current batch"
while pgrep -f "python3 scripts/gen_specs.py" >/dev/null 2>&1; do sleep 60; done
say "batch clear -- SDEI/DRTM/FFA finished"

cd "$R/repo" || exit 1
export HF_TOKEN=$(cat ~/.cache/huggingface/token)
O=$R/gen_docs; mkdir -p "$O/logs"
FT="--adapter jisenli/spec-check-ckpt --subfolder sft3-2/final"
i=0
for doc in scmi sbi; do
  for model in ft base; do
    for arm in nopre tail sel; do
      case $arm in
        nopre) A="" ;;
        tail)  A="--with-preamble --preamble-mode tail" ;;
        sel)   A="--with-preamble --preamble-mode selected" ;;
      esac
      M=""; [ "$model" = ft ] && M="$FT"
      tag="$doc-$model-$arm"; gpu=$((i % 8)); i=$((i+1))
      ( CUDA_VISIBLE_DEVICES=$gpu python3 scripts/gen_specs.py \
          --base Qwen/Qwen3.5-9B --prompt-variant v3.1 --versions "$doc" --no-gold \
          $M $A --out-dir "$O/$tag" > "$O/logs/$tag.log" 2>&1 ) &
      say "launched $tag on gpu$gpu"
      # 8 at a time: two 9B processes on one card will OOM.
      [ $((i % 8)) -eq 0 ] && wait
    done
  done
done
wait
say "ALL DONE -- 36 runs across 6 documents"
