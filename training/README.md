# spec-gen Training

Fine-tune Qwen3-4B to generate Verus formal specifications from Arm CCA RMM PDF sections.

## What this does

Given a chapter of the RMM specification PDF (text), the trained pipeline outputs a
`pub open spec fn {cmd}_spec(...)` Verus function body. The generation is split into
three cascaded layers:

| Layer | Input | Output | Model |
|-------|-------|--------|-------|
| L1 | — | type aliases, constants | hardcoded (`boilerplate/layer1.rs`) |
| L2 | PDF type section | `pub enum` / `struct` definitions | `models/layer2_fmt_best/` |
| L3 | PDF helper fn section | `pub open spec fn ...;` stubs | `models/layer3_fmt_best/` |
| CMD | PDF command section + L1–L3 context | `pub open spec fn {cmd}_spec(...)` | `models/commands_fmt_best/` |

L2 must be trained first; its outputs replace the golden context in CMD training
examples (cascaded training) to avoid covariate shift at inference time.

## Current Results (alp14 test set, 98 commands)

**Training data reformatted with `verusfmt` (Round 2, Apr 2026)**

| Metric | Round 2 (fmt models) | Round 1 (baseline) |
|--------|---------------------|--------------------|
| CodeBLEU | **0.416** | 0.637† |
| ngram_match | 0.151 | 0.406 |
| weighted_ngram | 0.236 | 0.427 |
| syntax_match | 0.561 | 0.812 |
| dataflow_match | 0.715 | 0.902 |
| Commands matched | **90 / 98** | 79 / 98 |

† Round 1 baseline evaluated against reformatted gold (not a clean comparison — see
[`verusfmt-retraining-results.md`](verusfmt-retraining-results.md) for details).

Coverage improved by +11 commands. The CodeBLEU drop is primarily caused by CMD model
overfitting (train_loss=0.094, eval_loss=0.903 after 10 epochs on 279 examples). Fix
in progress: early stopping / reduced epochs for CMD.

## Quick Start

```bash
# 1. Train L2 (type definitions)
python3 train.py \
    --train dataset/train_types.jsonl \
    --val   dataset/val_types.jsonl \
    --out   models/layer2_fmt

# 2. Generate cascaded context with trained L2
python3 inference_l2.py --model models/layer2_fmt_best
python3 substitute_context.py \
    --input   dataset/train.jsonl \
    --gen-dir generated_types/ \
    --output  dataset/train_cascaded.jsonl

# 3. Train L3 (helper stubs)
python3 train.py \
    --train dataset/train_helpers.jsonl \
    --val   dataset/val_helpers.jsonl \
    --out   models/layer3_fmt

# 4. Train CMD (command specs, cascaded)
python3 train.py \
    --train    dataset/train_cmds_cascaded.jsonl \
    --val      dataset/val_cmds.jsonl \
    --out      models/commands_fmt \
    --max-seq  6144 \
    --batch-size 2

# 5. Run pipeline on a new spec version
CUDA_VISIBLE_DEVICES=0 python3 pipeline.py \
    --sections-dir sections/alp14 \
    --target       alp14 \
    --l2-model     models/layer2_fmt_best \
    --l3-model     models/layer3_fmt_best \
    --cmd-model    models/commands_fmt_best \
    --out          alp14_generated.rs

# 6. Evaluate with CodeBLEU
python3 eval_codebleu.py
```

## Training Config

```
Base model:  unsloth/Qwen3-4B (4-bit QLoRA)
LoRA:        r=16, alpha=32, targets=q/k/v/o/gate/up/down_proj
Epochs:      10 (early stopping recommended for CMD)
LR:          2e-4, cosine scheduler, 3% warmup
Batch:       4 (L2/L3), 2 (CMD)  ×  grad_acc=4
max_seq:     4096 (L2/L3), 6144 (CMD)
GPU:         Quadro RTX 8000 48 GB, fp16, xformers attention
```

## Dataset

Training data covers 4 versions of the RMM spec (eac5, rel0, alp11, alp12);
validation on alp13; test on alp14.

| Split | Cmd | Type | Helper | Total |
|-------|-----|------|--------|-------|
| train | 279 | 179 | 480 | 938 |
| val (alp13) | 99 | 56 | 165 | 320 |
| test (alp14) | 98 | 57 | 171 | 326 |

All spec files are formatted with `verusfmt` as of Apr 2026.

## Files

| File | Purpose |
|------|---------|
| `train.py` | SFTTrainer + Unsloth QLoRA fine-tuning |
| `inference_l2.py` | Run L2 model over train-split type sections |
| `pipeline.py` | End-to-end inference (L2 → L3 → CMD → assembled .rs) |
| `build_dataset.py` | Build JSONL from spec files |
| `substitute_context.py` | Replace golden preamble with L2 output |
| `extract_sections.py` | Extract PDF text sections per command/type/helper |
| `split_specs.py` | Split gold .rs into per-command files |
| `test_e2e_oracle.py` | Oracle assembly smoke test (no GPU needed) |
| `STATUS.md` | Detailed training progress and per-epoch loss curves |
| `verusfmt-retraining-results.md` | Round 2 analysis and CodeBLEU breakdown |
| `README_training.md` | Extended training guide and dataset documentation |
| `boilerplate/layer1.rs` | Hardcoded L1 type aliases (not trained) |
