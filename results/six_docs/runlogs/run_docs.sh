#!/usr/bin/env bash
# 3 documents x 3 arms x 2 models = 18 runs, queued over 8 GPUs.
cd /shared/jisenli2/rulecheck/repo
export HF_TOKEN=$(cat ~/.cache/huggingface/token)
O=/shared/jisenli2/rulecheck/gen_docs; mkdir -p $O/logs
FT="--adapter jisenli/spec-check-ckpt --subfolder sft3-2/final"
i=0
for doc in sdei drtm ffa; do
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
          --base Qwen/Qwen3.5-9B --prompt-variant v3.1 --versions $doc --no-gold \
          $M $A --out-dir $O/$tag > $O/logs/$tag.log 2>&1 ) &
      echo "  gpu$gpu  $tag"
      # 8 at a time; more than one 9B per card will OOM.
      [ $((i % 8)) -eq 0 ] && wait
    done
  done
done
wait
echo "ALL DONE"
