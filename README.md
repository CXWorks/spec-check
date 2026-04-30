# spec-check

**Detecting Inconsistencies in Formally Verified Firmware Specifications** (ASPLOS 2026)

This repository contains two complementary approaches for converting firmware specification PDFs into [Verus](https://github.com/verus-lang/verus) formal verification code, plus an automated inconsistency checker that has found **5 machine-checked spec bugs** across ARM CCA RMM, SDEI, and DRTM specifications.

1. **SCOPE** — a hand-written 2,400-line Python pipeline (submodule)
2. **Fine-tuned LLM pipeline** — a Qwen3-4B model trained to replace SCOPE's heuristics

The generated Verus code encodes each command's preconditions and postconditions as `pub open spec fn` functions. A `proof fn ... ensures false` sweep then detects logical contradictions in the spec text.

---

## Key Results

### Spec Generation (alp14 test set, 98 RMM commands)

| Model | Epochs | Best epoch | Eval loss | CodeBLEU |
|-------|--------|------------|-----------|----------|
| Item-split 2-epoch (**current best**) | 2 | 2 | 0.2910 | **0.639** |
| Item-split 10-epoch (overfit) | 10 | 9 | 0.3809 | 0.594 |
| Round 2 — verusfmt-formatted | 10 | 1 | 0.7458 | 0.416 |
| Round 1 — baseline cascade | 10 | — | — | 0.637† |

† Round 1 evaluated against unformatted gold (not directly comparable to later rounds).

The item-split model trains all item types (commands, types, helpers) jointly without the 3-layer cascade, and uses early stopping at 2 epochs to avoid overfitting.

### Spec Bug Findings (machine-checked with Verus)

| Spec | Section | Bug type | Status |
|------|---------|----------|--------|
| ARM CCA RMM (alp14) | §B4.3.20.2.1 RMI_PDEV_STOP | Missing failure-condition ordering edge → dual error code contradiction | **Confirmed bug** |
| ARM CCA RMM (alp14) | §B5.3.1.2.1 RSI_ATTESTATION_TOKEN_CONTINUE | "No ordering" + two conflicting error clauses | **Confirmed bug** |
| ARM SDEI (DEN0054C) | §5.1.19 SDEI_SHARED_RESET | Contradictory precondition | **Confirmed bug** |
| ARM SDEI (DEN0054C) | §5.1.14 SDEI_INTERRUPT_BIND | Conflicting state requirement | **Confirmed bug** |
| ARM DRTM (DEN0113) | §3.11 DRTM_ENABLE_SECURE_INTERRUPTS | Dual error code contradiction | **Confirmed bug** |
| ARM CCA RMM (alp14) | §B5.3.19.2.1 RSI_VDEV_VALIDATE_MAPPING | Annotation false positive (spec is correct) | Annotation error |

See [`training/rmm_spec_bug_report.md`](training/rmm_spec_bug_report.md), [`training/spec_bug_report.md`](training/spec_bug_report.md), and [`training/BUG_REPORT.md`](training/BUG_REPORT.md) for details.

---

## Repository Structure

```
spec-check/
├── scope/                         # SCOPE tool (git submodule)
├── training/                      # LLM fine-tuning scripts and results
│   ├── train.py                   # Unsloth+Qwen3-4B SFTTrainer
│   ├── pipeline.py                # End-to-end inference pipeline (7 spec types)
│   ├── build_dataset.py           # Build JSONL datasets from PDF sections + gold specs
│   ├── extract_sections.py        # RMM section extractor
│   ├── extract_sections_{spec}.py # Per-spec extractors: psci, sdei, drtm, scmi, ffa, sbi, tdx
│   ├── cleanup_base.py            # Shared post-processing logic
│   ├── cleanup_{spec}.py          # Per-spec cleanup: psci, sdei, drtm, scmi, ffa, sbi, tdx
│   ├── eval_codebleu.py           # CodeBLEU evaluation against gold
│   ├── eval_item_split.py         # Item-split model comparison script
│   ├── inconsistency_analysis.py  # Automated Verus inconsistency sweep
│   ├── inconsistency_analysis_rmm.py  # RMM-specific sweep (with known-bug skip list)
│   ├── rmm_bugs.rs                # Machine-checked Verus proofs for RMM bugs
│   ├── spec_bugs.rs               # Machine-checked proofs for SDEI/DRTM bugs
│   ├── rmm_spec_bug_report.md     # Formal report for RMM spec bugs
│   ├── spec_bug_report.md         # Formal report for SDEI/DRTM bugs
│   ├── BUG_REPORT.md              # Summary of all 5 confirmed bugs
│   ├── boilerplate/               # Layer 1 type aliases and constants
│   │   ├── layer1.rs              # RMM
│   │   ├── layer1_psci.rs         # PSCI
│   │   ├── layer1_sdei.rs         # SDEI
│   │   ├── layer1_drtm.rs         # DRTM
│   │   ├── layer1_scmi.rs         # SCMI
│   │   ├── layer1_ffa.rs          # FF-A
│   │   ├── layer1_sbi.rs          # RISC-V SBI (new)
│   │   └── layer1_tdx.rs          # Intel TDX ABI (new)
│   ├── STATUS.md                  # Training history and lessons learned
│   └── README_training.md         # Full GPU training guide
└── training-dataset/              # Prepared dataset and gold specifications
    ├── dataset/                   # JSONL training/val/test splits
    ├── results/                   # Model predictions (item_split_e2, item_split_e10)
    ├── gold/                      # Complete gold Verus files for all 6 RMM versions
    ├── sections/{version}/        # Per-command/type/helper raw PDF text
    └── specs/{version}/           # Gold Verus split into per-command components
```

---

## Supported Specifications

### Training / Evaluation (RMM, gold annotations available)

| Version | Split |
|---------|-------|
| eac5 (1.0-eac5) | Train |
| rel0 (1.0-rel0) | Train |
| alp11 (1.1-alp11) | Train |
| alp12 (1.1-alp12) | Train |
| alp13 (1.1-alp13) | Val |
| alp14 (1.1-alp14) | **Test** |

### Zero-shot Extension (no gold annotations)

| Spec | Source | Analog | Status |
|------|--------|--------|--------|
| ARM PSCI (DEN0022F.b) | Arm public PDF | PSCI | Pipeline ready |
| ARM SDEI (DEN0054C) | Arm public PDF | — | Pipeline ready, 3 bugs found |
| ARM DRTM (DEN0113) | Arm public PDF | — | Pipeline ready, 1 bug found |
| ARM SCMI (DEN0056E) | Arm public PDF | — | Pipeline ready |
| ARM FF-A (DEN0077A) | Arm public PDF | — | Pipeline ready |
| RISC-V SBI v2.0 | RISC-V non-ISA GitHub | ARM PSCI | Extractor ready, PDF needed |
| Intel TDX ABI v1.5 | Intel CDN | ARM RMM | Extractor ready, PDF needed |

---

## Approach 1: SCOPE

SCOPE (*spectroscope*) is the baseline hand-written pipeline. See [`scope/README.md`](scope/README.md) and [`scope/USAGE.md`](scope/USAGE.md).

---

## Approach 2: LLM Fine-Tuning Pipeline

### Item-Split Training (current best)

The simplest and best-performing approach: train a single **Qwen3-4B** model on all item types (commands, types, helpers) jointly, using the shared per-item format without cascaded context:

```bash
# Train (2 epochs — early stopping critical)
python3 train.py \
    --train dataset/train.jsonl \
    --val   dataset/val.jsonl \
    --out   models/item_split \
    --epochs 2

# Evaluate
python3 eval_item_split.py
```

### Cascaded 3-Layer Pipeline (original architecture)

```
PDF → [L2 model] → type definitions  (pub enum / struct)
PDF → [L3 model] → helper stubs      (pub open spec fn ...;)
PDF + L2+L3 context → [CMD model]  → spec functions
```

See [`training/README_training.md`](training/README_training.md) for the full cascaded guide.

### Zero-Shot Extension to Other Specs

```bash
# Run on a new spec (e.g., RISC-V SBI) after PDF text extraction
python3 pipeline.py \
    --txt       ccaspec/sbi_2.txt \
    --target    sbi_2 \
    --l1-rs     boilerplate/layer1_sbi.rs \
    --cmd-model models/item_split_e2_best \
    --spec-type sbi \
    --out       sbi_generated.rs
python3 cleanup_sbi.py
```

### Inconsistency Checking

```bash
# Sweep all RMM gold specs for contradictions
python3 inconsistency_analysis_rmm.py

# Sweep generated specs from a new architecture
python3 inconsistency_analysis.py sbi   # checks sbi_generated_clean.rs
```

---

## License

SCOPE is licensed under Apache-2.0. See [`scope/LICENSE`](scope/LICENSE).
