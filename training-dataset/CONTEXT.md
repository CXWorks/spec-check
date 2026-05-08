# Project Context — spec-gen

This document summarizes the current project state for a new session or collaborator.

---

## What This Project Is

**spec-gen** trains an LLM to convert firmware specification PDFs into Verus formal verification code, then uses an automated inconsistency checker to find spec document bugs. Originally targeting ARM CCA RMM; now extended to 7 additional specs.

The Verus output encodes each firmware command's preconditions and postconditions as `pub open spec fn {cmd}_spec(...)` functions. A `proof fn ... ensures false` sweep detects logical contradictions.

---

## Current State (2026-04-29)

**Training complete.** Best model: `models/item_split_e2_best` (2-epoch item-split, CodeBLEU 0.639 on alp14).

**5 spec bugs confirmed** (machine-checked with Verus): 2 in ARM CCA RMM, 2 in ARM SDEI, 1 in ARM DRTM. See `training/BUG_REPORT.md`.

**Next step:** Zero-shot evaluation on RISC-V SBI and Intel TDX — requires downloading PDFs and extracting text.

---

## Directory Layout

```
training-dataset/
├── dataset/
│   ├── train.jsonl              # 1584 examples (eac5, rel0, alp11, alp12)
│   ├── val.jsonl                # 320 examples (alp13)
│   ├── test.jsonl               # 326 examples (alp14)
│   ├── train_helpers.jsonl      # 832 L3 helper examples (train split)
│   ├── train_cascaded.jsonl     # CMD examples with model-generated L2 context
│   ├── train_cmds_cascaded.jsonl  # CMD-only cascaded (476 examples)
│   ├── train_types.jsonl        # Type examples (276)
│   ├── val_cmds.jsonl           # Val commands (99)
│   ├── val_helpers.jsonl        # Val helpers (165)
│   └── val_types.jsonl          # Val types (56)
├── results/
│   ├── item_split_e2.jsonl      # Predictions from 2-epoch model (best)
│   └── item_split_e10.jsonl     # Predictions from 10-epoch model (overfit)
├── gold/                        # Complete gold Verus files for all 6 RMM versions
├── specs/{version}/             # Gold Verus split into per-command files
└── sections/{version}/          # Per-command/type/helper raw PDF text
```

---

## Spec Versions

| Split | Version | Description |
|-------|---------|-------------|
| train | eac5, rel0 | RMM 1.0 early alpha |
| train | alp11, alp12 | RMM 1.1 alpha |
| val | alp13 | RMM 1.1 alpha (held out during training) |
| **test** | **alp14** | **RMM 1.1 alpha (never trained on)** |

---

## Training Results Summary

| Model | Epochs | Eval loss | CodeBLEU |
|-------|--------|-----------|----------|
| `item_split_e2_best` | 2 | **0.2910** | **0.639** |
| `item_split_base_best` | 10 | 0.3809 | 0.594 |

The 2-epoch model is better by +4.5 CodeBLEU overall, +5 commands, +12.7 types.
10-epoch model overfits: eval loss rises monotonically from epoch 1 onward.

Config: `unsloth/Qwen3-4B`, LoRA r=16/alpha=32, lr=2e-4, cosine, batch 4×grad_acc 4, fp16, xformers, max_seq=6144.

---

## Running the Pipeline

```bash
# Zero-shot inference on a new spec
CUDA_VISIBLE_DEVICES=5 python3 pipeline.py \
    --txt       ccaspec/sbi_2.txt \
    --target    sbi_2 \
    --l1-rs     boilerplate/layer1_sbi.rs \
    --cmd-model models/item_split_e2_best \
    --spec-type sbi \
    --out       sbi_generated.rs
python3 cleanup_sbi.py

# Inconsistency sweep
python3 inconsistency_analysis_rmm.py    # full RMM sweep
python3 inconsistency_analysis.py sbi    # new spec

# Evaluate CodeBLEU (RMM only, gold available)
python3 eval_item_split.py
```

---

## Key Scripts

| Script | Purpose |
|--------|---------|
| `train.py` | Fine-tune with Unsloth + SFTTrainer |
| `pipeline.py` | PDF text → assembled .rs file |
| `build_dataset.py` | Build JSONL from section text + gold specs |
| `extract_sections.py` | Extract RMM PDF sections |
| `extract_sections_{spec}.py` | Per-spec extractors (psci/sdei/drtm/scmi/ffa/sbi/tdx) |
| `cleanup_base.py` + `cleanup_{spec}.py` | Post-process model output |
| `inconsistency_analysis.py` | Automated bug-finding sweep |
| `eval_item_split.py` | Compare model variants on CodeBLEU |

---

## Important Notes

- **alp14 is the test set** — never train on it
- **fp16 only** — Quadro RTX 8000 (Turing, CUDA 7.5) has no bf16 support
- **xformers required** for CMD sequences > 2k tokens; standard attention OOMs
- **2 epochs optimal** — 10 epochs always overfits at this data scale (~476 CMD examples)
- **`pub struct` in preamble** — inconsistency checker requires structs to be `pub` for standalone Verus crate compilation; `read_preamble()` applies this fix via regex
