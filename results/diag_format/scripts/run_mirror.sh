#!/usr/bin/env bash
# Mirror experiment: same weights, prompt and arm (nopre) as the six-document
# campaign ft-nopre runs, on PSCI and SDEI as-is, cleaned, and in RMM layout.
cd ~/spec-check
export HF_HUB_OFFLINE=1 TRANSFORMERS_OFFLINE=1
AD=$HOME/.cache/huggingface/hub/models--jisenli--spec-check-ckpt/snapshots/7b59ac04600bb54d60a4fc30e1e0c0dc25675992
COMMON="--base Qwen/Qwen3.5-9B --adapter $AD --subfolder sft3-2/final --prompt-variant v3.1 --no-gold"
mkdir -p ~/diag/gen2 ~/diag/logs
launch() { local g=$1 tag=$2; shift 2
  CUDA_VISIBLE_DEVICES=$g setsid nohup .venv/bin/python scripts/gen_specs.py $COMMON \
    --versions "$@" --out-dir ~/diag/gen2/$tag > ~/diag/logs/mirror-$tag.log 2>&1 < /dev/null &
  echo "gpu$g $tag pid=$!"; }
launch 0 ctl   psci_13 sdei
launch 1 clean psci_13_clean sdei_clean
launch 2 rmm   psci_13_rmm sdei_rmm
