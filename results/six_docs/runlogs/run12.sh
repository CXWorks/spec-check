#!/usr/bin/env bash
# SCMI + SBI: 2 documents x 3 arms x 2 models = 12 runs.
#
# Pulls first. The previous attempt did not, and the pod's checkout predated the
# commit that added SCMI/SBI sections, so all 12 exited in about a second with
# "0 commands loaded" -- and the wrapper still logged ALL DONE, because it only
# waited for the processes to end and never looked at why. Both fixed here:
# pull up front, and verify each run actually produced files.
set -u
R=/shared/jisenli2/rulecheck
LOG=$R/watchdog.log
say() { echo "[$(date '+%m-%d %H:%M:%S')] $*" >> "$LOG"; }

cd "$R/repo" || exit 1
git pull -q origin jisen/training || { say "git pull FAILED"; exit 1; }
say "repo @ $(git rev-parse --short HEAD)"
for d in scmi sbi; do
  n=$(ls training-dataset/sections/$d/*_command.txt 2>/dev/null | wc -l)
  [ "$n" -gt 0 ] || { say "PRECHECK FAILED: no sections for $d"; exit 1; }
  [ -f training-dataset/specs/$d/preamble.rs ] || { say "PRECHECK FAILED: no preamble for $d"; exit 1; }
  say "precheck $d: $n sections, preamble present"
done

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
      rm -rf "$O/$tag"
      ( CUDA_VISIBLE_DEVICES=$gpu python3 scripts/gen_specs.py \
          --base Qwen/Qwen3.5-9B --prompt-variant v3.1 --versions "$doc" --no-gold \
          $M $A --out-dir "$O/$tag" > "$O/logs/$tag.log" 2>&1 ) &
      say "launched $tag on gpu$gpu"
      [ $((i % 8)) -eq 0 ] && wait
    done
  done
done
wait

ok=0; bad=0
for doc in scmi sbi; do for model in ft base; do for arm in nopre tail sel; do
  tag="$doc-$model-$arm"
  n=$(ls "$O/$tag/$doc"/*.rs 2>/dev/null | wc -l)
  if [ "$n" -gt 0 ]; then ok=$((ok+1)); else bad=$((bad+1)); say "EMPTY: $tag"; fi
done; done; done
say "finished: $ok produced files, $bad empty"
