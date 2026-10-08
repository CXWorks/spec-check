#!/usr/bin/env bash
# tb_phase2.sh <ft|base> <ir version> <gpus csv> <versions...>
# Waits for the 9B's own tables (4 IR_DONE lines) and for Claude's tables
# (marker written after upload), then queues code generation with the full preamble.
m=$1; ir=$2; gpus=${3//,/ }; shift 3
until [ "$(cat ~/diag/logs/ir/$ir.*.log 2>/dev/null | grep -c "^IR_DONE $ir")" -ge 4 ] && [ -f ~/diag/tb_claude_ir_ready ]; do sleep 30; done
echo "tables ready: $ir $(ls ~/spec-check/training-dataset/sections/$ir | wc -l), claude $(ls ~/spec-check/training-dataset/sections/alp14_ir_claude | wc -l)"
exec ~/diag/launch_q.sh tb-$m --gpus $gpus --model $m --versions "$@" --chunk 7 --gen-args "--with-preamble --preamble-mode full"
