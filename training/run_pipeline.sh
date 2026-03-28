#!/usr/bin/env bash
# run_pipeline.sh — runs all fine-tuning steps sequentially on GPU 5
# Steps: L2 train → L2 inference → substitute → L3 train → filter cmds → CMD train
set -e
cd "$(dirname "$0")"

log() { echo "[$(date '+%H:%M:%S')] $*"; }

# ---------- Step 1: Train L2 (type definitions) ----------
log "Step 1: Training L2 (type definitions)..."
python3 train.py \
    --train dataset/train_types.jsonl \
    --val   dataset/val_types.jsonl \
    --out   models/layer2 \
    2>&1 | tee logs/layer2_train.log
log "Step 1 complete: models/layer2_best saved"

# ---------- Step 2a: L2 inference → generated_types/ ----------
log "Step 2a: Running L2 inference for cascaded context..."
python3 inference_l2.py 2>&1 | tee logs/inference_l2.log
log "Step 2a complete"

# ---------- Step 2b: substitute golden context with generated types ----------
log "Step 2b: Building train_cascaded.jsonl..."
python3 substitute_context.py \
    --input   dataset/train.jsonl \
    --gen-dir generated_types/ \
    --output  dataset/train_cascaded.jsonl
log "Step 2b complete: $(wc -l < dataset/train_cascaded.jsonl) examples in train_cascaded.jsonl"

# ---------- Step 3a: Train L3 (helper stubs) ----------
log "Step 3a: Training L3 (helper stubs)..."
python3 train.py \
    --train dataset/train_helpers.jsonl \
    --val   dataset/val_helpers.jsonl \
    --out   models/layer3 \
    2>&1 | tee logs/layer3_train.log
log "Step 3a complete: models/layer3_best saved"

# ---------- Step 3b: Filter command examples ----------
log "Step 3b: Filtering command examples..."
python3 - <<'EOF'
import json

# train_cmds_cascaded: command examples from cascaded file
with open('dataset/train_cascaded.jsonl') as f, \
     open('dataset/train_cmds_cascaded.jsonl', 'w') as g:
    for line in f:
        ex = json.loads(line)
        if ex['metadata'].get('kind') not in ('type_definition', 'helper_stub'):
            g.write(line)

# val_cmds: command examples from val split
with open('dataset/val.jsonl') as f, \
     open('dataset/val_cmds.jsonl', 'w') as g:
    for line in f:
        ex = json.loads(line)
        if ex['metadata'].get('kind') not in ('type_definition', 'helper_stub'):
            g.write(line)

import os
print(f"train_cmds_cascaded: {sum(1 for _ in open('dataset/train_cmds_cascaded.jsonl'))} examples")
print(f"val_cmds: {sum(1 for _ in open('dataset/val_cmds.jsonl'))} examples")
EOF
log "Step 3b complete"

# ---------- Step 4: Train CMD model ----------
log "Step 4: Training CMD model (max-seq=8192)..."
python3 train.py \
    --train   dataset/train_cmds_cascaded.jsonl \
    --val     dataset/val_cmds.jsonl \
    --out     models/commands \
    --max-seq 8192 \
    2>&1 | tee logs/commands_train.log
log "Step 4 complete: models/commands_best saved"

log "All training steps complete."
log "Next: run pipeline.py on alp14 for end-to-end evaluation."
