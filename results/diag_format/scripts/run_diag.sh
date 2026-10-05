#!/usr/bin/env bash
# Condition-format diagnostic. Same weights (sft3-2/final), same prompt (v3.1, no
# preamble -- the "nopre" arm of the 98-command run), same 49 held-out commands.
# Only the condition subsections differ between the three versions.
# Offline: the base model and the adapter are both in the HF cache, so no token.
cd ~/spec-check
export HF_HUB_OFFLINE=1 TRANSFORMERS_OFFLINE=1
AD=$HOME/.cache/huggingface/hub/models--jisenli--spec-check-ckpt/snapshots/7b59ac04600bb54d60a4fc30e1e0c0dc25675992
COMMON="--base Qwen/Qwen3.5-9B --adapter $AD --subfolder sft3-2/final --prompt-variant v3.1"
mkdir -p ~/diag/logs ~/diag/gen
for pair in "$@"; do                       # gpu:version, e.g. 0:alp14_ctl
  g=${pair%%:*}; v=${pair#*:}
  CUDA_VISIBLE_DEVICES=$g setsid nohup .venv/bin/python scripts/gen_specs.py \
    $COMMON --versions $v --out-dir ~/diag/gen \
    > ~/diag/logs/$v.log 2>&1 < /dev/null &
  echo "gpu$g  $v  pid=$!"
done
