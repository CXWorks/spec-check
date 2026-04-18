# Fine-tuning Status — spec-gen (Qwen3-4B + Unsloth)

**Last updated:** 2026-04-18
**GPU server:** cuda_device=5/6 (Quadro RTX 8000, 48 GB)
**Base model:** `unsloth/Qwen3-4B` (4-bit QLoRA)

---

## Training Progress

### Round 1 — Unformatted (baseline, ~2026-03-28)

| Layer | Task | Status | Model path | Notes |
|-------|------|--------|------------|-------|
| L2 | Type definitions | ✅ Done | `models/layer2_best/` | 179 train / 62 val |
| — | L2 inference → cascaded context | ✅ Done | `generated_types/` | 4 versions |
| — | substitute_context.py | ✅ Done | `dataset/train_cascaded.jsonl` | 938 total / 279 cmd |
| L3 | Helper stubs | ✅ Done | `models/layer3_best/` | 480 train / 159 val |
| CMD | Command specs (cascaded) | ✅ Done | `models/commands_best/` | train_loss=0.094, eval_loss=0.968 |
| Eval | alp14 CodeBLEU | ✅ Done | `alp14_generated.rs` | **0.632** (see below) |

### Round 2 — verusfmt-formatted training data (~2026-04-11 to 2026-04-16)

Gold spec files reformatted in-place with `verusfmt`, dataset rebuilt, all three models retrained.

| Layer | Task | Status | Model path | Notes |
|-------|------|--------|------------|-------|
| — | Format specs in-place | ✅ Done | `specs/{eac5..alp14}/**/*.rs` | Apr 11; skips verusfmt failures |
| — | Rebuild dataset | ✅ Done | `dataset/train.jsonl` | Same split sizes |
| L2 | Type definitions (fmt) | ✅ Done | `models/layer2_fmt_best/` | train_loss=0.365, eval_loss=0.107 |
| — | L2 fmt inference | ✅ Done | `generated_types/` | Re-generated with fmt model |
| — | substitute_context.py | ✅ Done | `dataset/train_cascaded.jsonl` | Rebuilt with fmt L2 context |
| L3 | Helper stubs (fmt) | ✅ Done | `models/layer3_fmt_best/` | train_loss=0.232, eval_loss=0.153 |
| CMD | Command specs (fmt, cascaded) | ✅ Done | `models/commands_fmt_best/` | train_loss=0.094, eval_loss=0.903 ⚠️ |
| Eval | alp14 CodeBLEU (fmt) | ✅ Done | `alp14_generated_fmt.rs` | **0.416** (see below) |

---

## CodeBLEU Results — alp14

> **Note on comparability:** The alp14 gold files were reformatted by verusfmt on Apr 11 (part of
> Round 2 prep). Both evals below use the same reformatted gold, so the baseline score (0.637) is
> slightly different from the originally reported 0.632 (which used unformatted gold).

| | CodeBLEU | ngram_match | weighted_ngram | syntax_match | dataflow_match | Matched |
|---|---|---|---|---|---|---|
| **Round 2 — fmt models** | **0.416** | 0.151 | 0.236 | 0.561 | 0.715 | 90 / 98 |
| Round 1 — baseline | 0.637 | 0.406 | 0.427 | 0.812 | 0.902 | 79 / 98 |

Coverage improved (+11 commands matched), but quality scores dropped. Root cause: CMD model
overfitting (see below). Full breakdown in `verusfmt-retraining-results.md`.

---

## Training Configuration

```
Model:      unsloth/Qwen3-4B (load_in_4bit=True)
LoRA:       r=16, alpha=32
Targets:    q/k/v/o/gate/up/down_proj
Epochs:     10
Batch:      4 (grad_acc=4, effective=16) for L2/L3
            2 (grad_acc=4, effective=8)  for CMD (longer sequences)
LR:         2e-4, cosine scheduler, 3% warmup
Precision:  fp16 (GPU is Turing, no bf16 support)
Attn:       xformers 0.0.35 (required for CMD — standard attn OOMs at seq>2k)
max_seq:    4096 for L2/L3 | 6144 for CMD (covers 89% of examples without truncation)
Grad ckpt:  gradient_checkpointing=True + use_reentrant=False (CMD only)
```

---

## Key Lessons Learned

1. **xformers is required for CMD training.** Without it, standard O(seq²) attention OOMs even
   with batch=1 on CMD examples (median 4114 tokens). VRAM dropped from 36 GB → 12 GB after
   installing xformers.

2. **`use_gradient_checkpointing="unsloth"` causes OOM on this GPU.** Unsloth's custom gradient
   checkpointing unexpectedly inflated memory usage on the Quadro RTX 8000 (Turing, CUDA 7.5,
   no FlashAttention). Standard PyTorch `gradient_checkpointing=True` works correctly.

3. **CMD needs separate settings from L2/L3.** L2/L3 examples are short (100–2000 tokens), CMD
   examples are long (median 4114, max 9128). Different batch size and gradient checkpointing needed.

4. **Inference_l2.py system prompt must match build_dataset.py exactly.** Originally used a shorter
   generic prompt; fixed to `SYSTEM_PROMPT_TYPES` from `build_dataset.py` before running inference.

5. **CMD model overfits severely at 10 epochs with 279 examples.** Round 2 CMD: train_loss=0.094,
   eval_loss monotonically increased epoch 1→10 (0.746→0.903). Optimal checkpoint is around epoch
   1. Fix: use early stopping or reduce epochs to 3–4.

6. **apply_chat_template may return a BatchEncoding, not a tensor.** Unsloth tokenizer wraps the
   output in a `BatchEncoding` when called with `return_tensors="pt"`. Must check
   `hasattr(raw, "input_ids")` before calling `.to(device)`.

---

## Open Issues / Next Steps

1. **Fix CMD overfitting:** Retrain CMD with early stopping at epoch 1–2, or reduce epochs, or
   add weight_decay. Expected to recover most of the CodeBLEU drop.

2. **Clean apples-to-apples comparison:** Run baseline eval against unformatted gold (pre-Apr-11
   backup) to isolate the formatter effect from the model quality change.

3. **Diagnose worst commands:** `RSI_PLANE_SYSREG_READ` (0.183), `PSCI_FEATURES` (0.210) — check
   whether these are structurally novel or have missing preamble context.

---

## File Map

```
spec-gen/
├── train.py                        # Unsloth+Qwen3 SFTTrainer (L2/L3/CMD)
├── inference_l2.py                 # Run L2 model → generated_types/ (--model arg)
├── pipeline.py                     # End-to-end inference (implemented; fmt_code() added)
├── build_dataset.py                # Builds JSONL datasets from PDF sections + gold specs
├── extract_sections.py             # Extracts PDF text sections per command/type/helper
├── split_specs.py                  # Splits gold .rs into per-command files
├── substitute_context.py           # Replaces golden preamble with L2-generated context
├── test_e2e_oracle.py              # Oracle assembly test (validates pipeline without GPU)
├── run_pipeline.sh                 # Chains all training steps (L2→inference→L3→CMD)
├── run_cascade_and_cmd.sh          # Wait-for-L2/L3, then run cascade+CMD
├── README_training.md              # Full training guide
├── CONTEXT.md                      # Project overview and architecture
├── verusfmt-retraining-results.md  # Detailed Round 2 results and analysis
└── boilerplate/layer1.rs           # Hardcoded L1 type aliases (never trained)
```
