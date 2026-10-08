#!/usr/bin/env bash
# ir_launch.sh <ft|base> <version> <gpu...>: one detached shard per GPU.
m=$1; v=$2; shift 2; n=$#; k=0
AD=$HOME/.cache/huggingface/hub/models--jisenli--spec-check-ckpt/snapshots/7b59ac04600bb54d60a4fc30e1e0c0dc25675992
mkdir -p ~/diag/logs/ir
for g in "$@"; do
  (cd ~/spec-check && HF_HUB_OFFLINE=1 TRANSFORMERS_OFFLINE=1 CUDA_VISIBLE_DEVICES=$g setsid nohup \
    .venv/bin/python scripts/ir_testbed.py ninebee --model $m --adapter $AD \
    --sections training-dataset/sections/alp14_prose --out training-dataset --version $v \
    --shard $k --nshards $n > ~/diag/logs/ir/$v.$k.log 2>&1 < /dev/null &)
  echo "gpu$g $v shard $k/$n"; k=$((k+1))
done
