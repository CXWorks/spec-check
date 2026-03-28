#!/usr/bin/env bash
# run_cascade_and_cmd.sh
# Waits for L2 and L3 to finish, then runs:
#   Step 2: L2 inference + substitute_context
#   Step 3b: filter CMD examples
#   Step 4: CMD training
set -e
cd "$(dirname "$0")"

log() { echo "[$(date '+%H:%M:%S')] $*"; }

# ---------- Wait for L2 to finish ----------
log "Waiting for L2 model (models/layer2_best)..."
while [ ! -d models/layer2_best ]; do sleep 30; done
log "L2 ready."

# ---------- Step 2a: L2 inference ----------
log "Step 2a: Running L2 inference → generated_types/..."
python3 inference_l2.py 2>&1 | tee logs/inference_l2.log
log "Step 2a complete."

# ---------- Step 2b: substitute context ----------
log "Step 2b: Building train_cascaded.jsonl..."
python3 substitute_context.py \
    --input   dataset/train.jsonl \
    --gen-dir generated_types/ \
    --output  dataset/train_cascaded.jsonl
log "Step 2b: $(wc -l < dataset/train_cascaded.jsonl) examples in train_cascaded.jsonl"

# ---------- Wait for L3 to finish ----------
log "Waiting for L3 model (models/layer3_best)..."
while [ ! -d models/layer3_best ]; do sleep 30; done
log "L3 ready."

# ---------- Step 3b: filter CMD examples ----------
log "Step 3b: Filtering command examples..."
python3 - <<'EOF'
import json

with open('dataset/train_cascaded.jsonl') as f, \
     open('dataset/train_cmds_cascaded.jsonl', 'w') as g:
    for line in f:
        ex = json.loads(line)
        if ex['metadata'].get('kind') not in ('type_definition', 'helper_stub'):
            g.write(line)

with open('dataset/val.jsonl') as f, \
     open('dataset/val_cmds.jsonl', 'w') as g:
    for line in f:
        ex = json.loads(line)
        if ex['metadata'].get('kind') not in ('type_definition', 'helper_stub'):
            g.write(line)

n_train = sum(1 for _ in open('dataset/train_cmds_cascaded.jsonl'))
n_val   = sum(1 for _ in open('dataset/val_cmds.jsonl'))
print(f"train_cmds_cascaded: {n_train} | val_cmds: {n_val}")
EOF
log "Step 3b complete."

# ---------- Step 4: CMD training ----------
log "Step 4: Training CMD model (max-seq=8192)..."
python3 train.py \
    --train   dataset/train_cmds_cascaded.jsonl \
    --val     dataset/val_cmds.jsonl \
    --out     models/commands \
    --max-seq 8192 \
    2>&1 | tee logs/commands_train.log
log "Step 4 complete: models/commands_best saved."

log "All done. Run pipeline.py on alp14 for end-to-end evaluation."
