#!/usr/bin/env bash
# PSCI, 22 commands, 2 models x 3 arms, one GPU each.
# Repair arms are omitted: they are Verus-driven, and on alp14 each repair arm
# produced the same dangling-output set as its non-repair counterpart, because
# an unconstrained output is not a compile error.
cd /shared/jisenli2/rulecheck/repo
export HF_TOKEN=$(cat ~/.cache/huggingface/token)
OUT=/shared/jisenli2/rulecheck/gen_psci
mkdir -p $OUT/logs
COMMON="--base Qwen/Qwen3.5-9B --prompt-variant v3.1 --versions psci_13 --no-gold"
FT="--adapter jisenli/spec-check-ckpt --subfolder sft3-2/final"

launch() { # gpu tag model_args arm_args
  local gpu=$1 tag=$2; shift 2
  CUDA_VISIBLE_DEVICES=$gpu nohup python3 scripts/gen_specs.py $COMMON "$@" \
    --out-dir $OUT/$tag > $OUT/logs/$tag.log 2>&1 &
  echo "  gpu$gpu  $tag  pid=$!"
}
launch 0 ft-nopre   $FT
launch 1 ft-tail    $FT --with-preamble --preamble-mode tail
launch 2 ft-sel     $FT --with-preamble --preamble-mode selected
launch 3 base-nopre
launch 4 base-tail  --with-preamble --preamble-mode tail
launch 5 base-sel   --with-preamble --preamble-mode selected
echo "launched 6 runs"
