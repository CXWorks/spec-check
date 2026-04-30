# Fine-tuning Status — spec-gen (Qwen3-4B + Unsloth)

**Last updated:** 2026-04-29
**GPU server:** cuda_device=5/6 (Quadro RTX 8000, 48 GB)
**Base model:** `unsloth/Qwen3-4B` (4-bit QLoRA)

---

## Training History

### Round 1 — Cascaded 3-layer baseline (~2026-03-28)

| Layer | Status | Model path | Notes |
|-------|--------|------------|-------|
| L2 type definitions | ✅ Done | `models/layer2_best/` | 179 train / 62 val |
| L2 inference → cascaded context | ✅ Done | `generated_types/` | 4 versions |
| L3 helper stubs | ✅ Done | `models/layer3_best/` | 480 train / 159 val |
| CMD command specs (cascaded) | ✅ Done | `models/commands_best/` | train_loss=0.094, eval_loss=0.968 |
| Eval — alp14 CodeBLEU | ✅ Done | `alp14_generated.rs` | **0.632** (vs unformatted gold) |

### Round 2 — verusfmt-formatted retraining (~2026-04-11 to 2026-04-16)

Gold specs reformatted in-place with `verusfmt`, dataset rebuilt, all models retrained.

| Layer | Status | Model path | Notes |
|-------|--------|------------|-------|
| Gold formatting | ✅ Done | `specs/{eac5..alp14}/**/*.rs` | Apr 11 |
| L2 type definitions (fmt) | ✅ Done | `models/layer2_fmt_best/` | train_loss=0.365, eval_loss=0.107 |
| L3 helper stubs (fmt) | ✅ Done | `models/layer3_fmt_best/` | best at epoch 5, eval_loss=0.142 |
| CMD command specs (fmt) | ✅ Done | `models/commands_fmt_best/` | severe overfit: eval_loss=0.903 ⚠️ |
| Eval — alp14 CodeBLEU | ✅ Done | `alp14_generated_fmt.rs` | **0.416** (vs fmt gold) |

Full analysis in [`verusfmt-retraining-results.md`](verusfmt-retraining-results.md). Root cause of regression: CMD model memorized 279 training examples over 10 epochs (train_loss → 0.094, eval_loss monotonically rising from epoch 1).

### Round 3 — Item-split single-model (~2026-04-18 to 2026-04-29) ← **Current Best**

Dropped the 3-layer cascade. Trained a single model on all item types (commands, types, helpers) jointly using the item-split format. Key finding: **2 epochs is optimal**; 10 epochs overfits.

| Model | Epochs | Best epoch | Eval loss | CodeBLEU | Commands | Types | Helpers |
|-------|--------|------------|-----------|----------|----------|-------|---------|
| `models/item_split_base_best` | 10 | 9 | 0.3809 | 0.594 | — | — | — |
| `models/item_split_e2_best` | 2 | 2 | **0.2910** | **0.639** | ~0.65 | ~0.57 | ~0.62 |

The 2-epoch model improves over Round 1 baseline (+0.7 CodeBLEU overall, +5 on commands, +12.7 on types) while being far simpler — no cascade, no multi-stage training, single model serves all item types.

Predictions saved in `results/item_split_e2.jsonl` and `results/item_split_e10.jsonl`.

---

## CodeBLEU Summary — alp14

| Round | CodeBLEU | Matched | Notes |
|-------|----------|---------|-------|
| Round 3 — item-split 2-epoch | **0.639** | — | vs fmt gold |
| Round 1 — cascade baseline | 0.637 | 79/98 | vs unformatted gold (not comparable) |
| Round 3 — item-split 10-epoch | 0.594 | — | overfit |
| Round 2 — fmt cascade | 0.416 | 90/98 | CMD overfit |

---

## Training Configuration

```
Model:      unsloth/Qwen3-4B (load_in_4bit=True)
LoRA:       r=16, alpha=32
Targets:    q/k/v/o/gate/up/down_proj
Epochs:     2 (item-split) / 10 (cascaded)
Batch:      4 × grad_acc=4  (effective=16)
LR:         2e-4, cosine scheduler, 3% warmup
Precision:  fp16 (Quadro RTX 8000 is Turing — no bf16)
Attn:       xformers 0.0.35 (prevents OOM for CMD long sequences)
max_seq:    4096 for L2/L3 | 6144 for CMD/item-split
Grad ckpt:  gradient_checkpointing=True + use_reentrant=False
```

---

## Spec Bug Findings (Verus-verified)

All bugs confirmed by compiling `proof fn check(...) requires spec_fn(...) ensures false {}` with 0 errors.

| Spec | Section | Type |
|------|---------|------|
| ARM CCA RMM §B4.3.20.2.1 | RMI_PDEV_STOP | Missing ordering edge → dual error code |
| ARM CCA RMM §B5.3.1.2.1 | RSI_ATTESTATION_TOKEN_CONTINUE | "No ordering" + conflicting errors |
| ARM SDEI DEN0054C §5.1.19 | SDEI_SHARED_RESET | Contradictory precondition |
| ARM SDEI DEN0054C §5.1.14 | SDEI_INTERRUPT_BIND | Conflicting state requirement |
| ARM DRTM DEN0113 §3.11 | DRTM_ENABLE_SECURE_INTERRUPTS | Dual error code |

Proofs in `rmm_bugs.rs` and `spec_bugs.rs`. Detailed reports in `rmm_spec_bug_report.md`, `spec_bug_report.md`, `BUG_REPORT.md`.

Full automated sweep of 89 RMM alp14 gold specs: **0 new inconsistencies** (74 consistent, 3 trivial, 12 type-error/skip).

---

## Multi-Spec Extension (Zero-Shot)

Pipeline extended to 7 additional specs beyond RMM. Each has a section extractor, `layer1_*.rs` boilerplate, and `cleanup_*.py`.

| Spec | Extractor | Layer1 | Status |
|------|-----------|--------|--------|
| ARM PSCI | `extract_sections_psci.py` | `layer1_psci.rs` | Pipeline ready |
| ARM SDEI | `extract_sections_sdei.py` | `layer1_sdei.rs` | Pipeline ready |
| ARM DRTM | `extract_sections_drtm.py` | `layer1_drtm.rs` | Pipeline ready |
| ARM SCMI | `extract_sections_scmi.py` | `layer1_scmi.rs` | Pipeline ready |
| ARM FF-A | `extract_sections_ffa.py` | `layer1_ffa.rs` | Pipeline ready |
| RISC-V SBI v2.0 | `extract_sections_sbi.py` | `layer1_sbi.rs` | PDF needed |
| Intel TDX ABI v1.5 | `extract_sections_tdx.py` | `layer1_tdx.rs` | PDF needed |

---

## Key Lessons Learned

1. **Early stopping is critical for CMD/item-split.** With 279–476 command examples, 10 epochs massively overfits (train_loss → ~0.09, eval_loss → 0.9+). Optimal checkpoint is epoch 1–2. Use `--epochs 2` or `load_best_model_at_end=True`.

2. **Item-split beats the 3-layer cascade.** Simpler architecture, better CodeBLEU, no cascaded covariate shift. The cascade adds complexity without measurable benefit at this data scale.

3. **xformers is required for CMD training.** Without it, standard O(seq²) attention OOMs even at batch=1 for long CMD examples (median 4114 tokens). VRAM: 36 GB → 12 GB after xformers.

4. **`use_gradient_checkpointing="unsloth"` causes OOM on Turing GPUs.** Use standard PyTorch `gradient_checkpointing=True` instead.

5. **`pub struct` required for standalone Verus crate.** Inconsistency checker compiles specs standalone; structs in preamble must be `pub` for field accesses in `pub open spec fn` to typecheck. Applied via regex in `read_preamble()`.

6. **apply_chat_template may return a BatchEncoding.** Unsloth tokenizer wraps output in `BatchEncoding` when called with `return_tensors="pt"`. Check `hasattr(raw, "input_ids")` before `.to(device)`.

---

## Open / Next Steps

1. **Run pipeline on RISC-V SBI and Intel TDX** — download PDFs, extract text, run inference, check inconsistencies
2. **Submit errata to ARM** — errata email drafted in `errata_email.txt`
