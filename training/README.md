# spec-gen Training

Fine-tune Qwen3-4B to generate Verus formal specifications from firmware spec PDF sections.

## What this does

Given a chapter of a firmware specification PDF (text), the trained pipeline outputs a
`pub open spec fn {cmd}_spec(...)` Verus function body encoding preconditions and
postconditions. The same model is used for zero-shot transfer to new firmware specs (PSCI,
SDEI, DRTM, SCMI, FF-A, RISC-V SBI, Intel TDX).

## Current Results (alp14 test set, 98 RMM commands)

| Model | Epochs | Eval loss | CodeBLEU |
|-------|--------|-----------|----------|
| **Item-split 2-epoch (current best)** | 2 | 0.2910 | **0.639** |
| Item-split 10-epoch (overfit) | 10 | 0.3809 | 0.594 |
| Round 2 — verusfmt cascade | 10 | 0.903 | 0.416 |
| Round 1 — baseline cascade | 10 | 0.968 | 0.637† |

† Round 1 evaluated against unformatted gold (not directly comparable).

Key finding: **training for 2 epochs is optimal**. The 10-epoch model memorizes training examples; the 2-epoch model generalizes better despite lower training loss.

See [`STATUS.md`](STATUS.md) for full training history and [`verusfmt-retraining-results.md`](verusfmt-retraining-results.md) for Round 2 analysis.

## Quick Start (Item-Split — Recommended)

```bash
pip install unsloth trl peft transformers datasets accelerate bitsandbytes xformers

# Train single model on all item types (2 epochs)
python3 train.py \
    --train dataset/train.jsonl \
    --val   dataset/val.jsonl \
    --out   models/item_split \
    --epochs 2 \
    --max-seq 6144

# Evaluate on alp14 test set
python3 eval_item_split.py

# Run pipeline on a new spec (zero-shot)
CUDA_VISIBLE_DEVICES=0 python3 pipeline.py \
    --sections-dir sections/alp14 \
    --target       alp14 \
    --cmd-model    models/item_split_e2_best \
    --spec-type    rmm \
    --out          alp14_generated.rs
```

## Cascaded 3-Layer Pipeline (original architecture)

```
PDF → [L2 model] → type definitions  (pub enum / struct)
PDF → [L3 model] → helper stubs      (pub open spec fn ...;)
PDF + L2+L3 context → [CMD model]  → spec functions
```

See [`README_training.md`](README_training.md) for the full cascaded guide.

## Training Config

```
Base model:  unsloth/Qwen3-4B (4-bit QLoRA)
LoRA:        r=16, alpha=32, targets=q/k/v/o/gate/up/down_proj
Epochs:      2 (item-split), early stopping critical
LR:          2e-4, cosine scheduler, 3% warmup
Batch:       4 × grad_acc=4  (effective batch 16)
max_seq:     6144
GPU:         Quadro RTX 8000 48 GB, fp16, xformers attention
```

## Dataset (RMM, 6 versions)

| Split | Versions | Cmd | Type | Helper | Total |
|-------|----------|-----|------|--------|-------|
| train | eac5, rel0, alp11, alp12 | 476 | 276 | 832 | 1584 |
| val | alp13 | 99 | 56 | 165 | 320 |
| test | alp14 | 98 | 57 | 171 | 326 |

## Spec Bug Findings

The automated inconsistency checker found **5 confirmed spec document bugs**:

| Spec | Bug |
|------|-----|
| ARM CCA RMM §B4.3.20.2.1 | RMI_PDEV_STOP: missing ordering edge → dual error |
| ARM CCA RMM §B5.3.1.2.1 | RSI_ATTESTATION_TOKEN_CONTINUE: "no ordering" + conflicting errors |
| ARM SDEI DEN0054C §5.1.19 | SDEI_SHARED_RESET: contradictory precondition |
| ARM SDEI DEN0054C §5.1.14 | SDEI_INTERRUPT_BIND: conflicting state requirement |
| ARM DRTM DEN0113 §3.11 | DRTM_ENABLE_SECURE_INTERRUPTS: dual error code |

```bash
# Reproduce bug sweep
python3 inconsistency_analysis_rmm.py    # RMM sweep (0 new bugs in 74 specs)
python3 inconsistency_analysis.py        # Other specs
```

## Multi-Spec Extension

`pipeline.py` supports 8 spec types via `--spec-type`:

```bash
python3 pipeline.py --spec-type {rmm|psci|sdei|drtm|scmi|ffa|sbi|tdx} ...
```

Each spec type has a corresponding `extract_sections_{spec}.py`, `cleanup_{spec}.py`, and `boilerplate/layer1_{spec}.rs`.

## Files

| File | Purpose |
|------|---------|
| `train.py` | SFTTrainer + Unsloth QLoRA fine-tuning |
| `pipeline.py` | End-to-end inference (multi-spec, 8 spec types) |
| `build_dataset.py` | Build JSONL from spec files |
| `eval_codebleu.py` | CodeBLEU evaluation |
| `eval_item_split.py` | Item-split model comparison |
| `inconsistency_analysis.py` | Automated Verus inconsistency sweep |
| `inconsistency_analysis_rmm.py` | RMM-specific sweep with known-bug skip list |
| `rmm_bugs.rs` / `spec_bugs.rs` | Machine-checked Verus proofs for confirmed bugs |
| `extract_sections_*.py` | Per-spec PDF section extractors (8 specs) |
| `cleanup_base.py` + `cleanup_*.py` | Post-processing model output (7 non-RMM specs) |
| `boilerplate/layer1*.rs` | Hardcoded type aliases per spec (8 files) |
| `STATUS.md` | Training history, per-epoch loss, lessons learned |
| `verusfmt-retraining-results.md` | Round 2 detailed analysis |
| `README_training.md` | Extended cascaded training guide |
