# verusfmt-Formatted Retraining: Results & Analysis

**Date:** 2026-04-16
**Baseline eval date:** ~2026-04-07
**Eval target:** alp14 (98 gold command specs)

---

## Motivation

The baseline pipeline (CodeBLEU 0.632) applied `verusfmt` only at **eval time**, not during training.
This created a train/inference distribution mismatch: models learned unformatted whitespace/indentation
patterns, but outputs were compared to verusfmt-normalized gold.

The hypothesis was that retraining on verusfmt-formatted spec outputs would align the training
distribution with the eval format, improving ngram-based metrics.

---

## Changes Made

### 1. Gold spec files reformatted in-place (2026-04-11)

All `.rs` files under `specs/{eac5,rel0,alp11,alp12,alp13,alp14}/` were formatted with
`verusfmt/target/release/verusfmt`. Files where verusfmt exited non-zero were left unchanged.
This affects:

- `specs/{version}/types/*.rs` — L2 training targets
- `specs/{version}/helpers/*.rs` — L3 training targets
- `specs/{version}/*_spec.rs` — CMD training targets and alp14 eval gold

> **Note:** Because the alp14 gold files were also reformatted, the new eval results are **not
> directly comparable** to the pre-Apr-11 baseline. The old baseline (0.632) was computed against
> unformatted gold; the new eval uses reformatted gold.

### 2. Dataset rebuilt from formatted specs

`build_dataset.py` re-run to regenerate `dataset/train.jsonl` and `dataset/val.jsonl` from the
now-formatted spec files.

### 3. New models trained (`*_fmt_best`)

Three new models trained with the same hyperparameters as the originals but on formatted data.
Old models (`models/layer2_best`, `models/layer3_best`, `models/commands_best`) kept intact.

### 4. `pipeline.py` updated

- Added `fmt_code()` helper: wraps snippet in `verus! {}`, runs `verusfmt --verus-only`, strips
  wrapper, falls back to original on error.
- `fmt_code()` applied to each L2 (type) and L3 (helper) generation output before they are
  concatenated into the CMD model's preamble context.
- Added `--sections-dir` CLI argument (bypasses PDF extraction for pre-extracted section dirs).
- Implemented `load_model()` / `run_model()` with Unsloth + Qwen3-4B.

---

## Model Training Details

All models: **Qwen3-4B**, QLoRA (Unsloth), 4-bit quantization, single GPU (Quadro RTX 8000, 48 GB).

### Hyperparameters (shared)

| Parameter | Value |
|---|---|
| LoRA rank (r) | 16 |
| LoRA alpha | 32 |
| Target modules | q_proj, k_proj, v_proj, o_proj |
| Learning rate | 2e-4 |
| LR scheduler | cosine |
| Warmup ratio | 0.03 |
| Gradient accumulation steps | 4 |
| Epochs | 10 |

CMD model used `--max-seq 6144`, `--batch-size 2` (larger context for preamble).

### Dataset sizes

| Split | Examples |
|---|---|
| train_types | 179 |
| val_types | 62 |
| train_helpers | 480 |
| val_helpers | 159 |
| train_cmds_cascaded | 279 |
| val_cmds | 99 |

CMD training uses cascaded context: the CMD examples' preamble field was replaced with L2-generated
types (from `inference_l2.py` with `models/layer2_fmt_best`) via `substitute_context.py`.

### Training curves

#### L2 — Type definitions (`models/layer2_fmt_best`)

| Epoch | Eval Loss |
|---|---|
| 1 | 0.7826 |
| 2 | 0.4804 |
| 3 | 0.3218 |
| 4 | 0.2187 |
| 5 | 0.1591 |
| 6 | 0.1287 |
| 7 | 0.1135 |
| 8 | 0.1088 |
| 9 | 0.1074 |
| **10** | **0.1068** |

**Final:** train_loss=0.3653, eval_loss=0.1068 — healthy convergence, no overfitting.

#### L3 — Helper function stubs (`models/layer3_fmt_best`)

| Epoch | Eval Loss |
|---|---|
| 1 | 0.4521 |
| 2 | 0.2221 |
| 3 | 0.1600 |
| 4 | 0.1517 |
| **5** | **0.1419** ← best |
| 6 | 0.1467 |
| 7 | 0.1506 |
| 8 | 0.1507 |
| 9 | 0.1522 |
| 10 | 0.1526 |

**Final:** train_loss=0.2315, eval_loss=0.1526 — slight overfitting after epoch 5.

#### CMD — Command specs (`models/commands_fmt_best`)

| Epoch | Eval Loss |
|---|---|
| 1 | 0.7458 |
| 2 | 0.7499 |
| 3 | 0.7800 |
| 4 | 0.7857 |
| 5 | 0.8012 |
| 6 | 0.8465 |
| 7 | 0.8696 |
| 8 | 0.8857 |
| 9 | 0.9008 |
| 10 | 0.9028 |

**Final:** train_loss=0.0941, eval_loss=0.9028 — **severe overfitting**. Eval loss monotonically
increases from epoch 1 while train loss drops to near zero. The model memorized training commands
but did not generalize to held-out examples.

---

## CodeBLEU Evaluation Results

Evaluated on alp14 (98 gold commands), gold files reformatted with verusfmt (as of Apr 11).

### Aggregate scores

| Model | CodeBLEU | ngram_match | weighted_ngram | syntax_match | dataflow_match | Matched |
|---|---|---|---|---|---|---|
| **Formatted (new)** | 0.4155 | 0.1508 | 0.2355 | 0.5605 | 0.7152 | 90 / 98 |
| Unformatted (baseline†) | 0.6370 | 0.4064 | 0.4269 | 0.8121 | 0.9024 | 79 / 98 |

† Baseline (`alp14_generated.rs`, Apr 7) evaluated against the same reformatted gold — not a
clean apples-to-apples comparison since the baseline was generated before gold reformatting.

### Per-command breakdown (formatted models)

**Worst 10:**

| Score | Command |
|---|---|
| 0.183 | RSI_PLANE_SYSREG_READ |
| 0.210 | PSCI_FEATURES |
| 0.227 | RSI_MEASUREMENT_READ |
| 0.260 | RMI_RTT_DESTROY |
| 0.264 | RMI_RTT_SET_S2AP |
| 0.287 | RSI_VSMMU_ACTIVATE |
| 0.297 | RMI_VDEV_VALIDATE_MAPPING |
| 0.312 | RMI_VDEV_P2P_BIND |
| 0.314 | RMI_RTT_INIT_RIPAS |
| 0.324 | PSCI_SYSTEM_OFF |

**Best 10:**

| Score | Command |
|---|---|
| 0.768 | RMI_PSMMU_MSI_CONFIG |
| 0.648 | RSI_ATTESTATION_TOKEN_INIT |
| 0.629 | RMI_VDEV_AUX_COUNT |
| 0.623 | RMI_VDEV_DESTROY |
| 0.616 | RMI_RTT_AUX_DESTROY |
| 0.604 | RMI_RTT_AUX_CREATE |
| 0.563 | RMI_VSMMU_CREATE |
| 0.549 | RSI_REALM_CONFIG |
| 0.536 | RMI_REC_AUX_COUNT |
| 0.534 | PSCI_AFFINITY_INFO |

**8 unmatched commands** (gold-only — not extracted by pipeline):
`RMI_MEC_SET_PRIVATE`, `RMI_MEC_SET_SHARED`, `RMI_PDEV_IDE_RESET`, `RMI_PDEV_P2P_DISCONNECT`,
`RMI_RTT_AUX_UNMAP_UNPROTECTED` (and 3 others).

---

## Analysis

### Coverage improved, quality degraded

The formatted models matched **90/98** commands vs **79/98** for the baseline — a gain of 11
commands. However, all CodeBLEU sub-scores dropped substantially.

### Root causes

1. **CMD model overfitting (primary cause):** Train loss dropped to 0.094 while eval loss rose
   monotonically to 0.903. With only 279 training examples and 10 epochs, the model memorized
   training commands verbatim. The optimal checkpoint was likely around epoch 1 (eval_loss=0.746).

2. **Incomparable baselines:** The old baseline was generated before the gold files were reformatted
   (Apr 7 vs Apr 11). The old baseline evaluated against unformatted gold; the new eval uses
   verusfmt-normalized gold. The structural change in gold means some of the apparent score drop
   reflects the formatter change, not model quality alone.

3. **Cascaded errors:** CMD model receives L2/L3 outputs as context. If L2/L3 generate types/helpers
   that differ from gold, the CMD model sees an unfamiliar preamble, degrading its output.

### Recommended next steps

1. **Fix CMD overfitting:** Use early stopping (best checkpoint at epoch ~1), reduce epochs to 3–4,
   or increase `weight_decay` (e.g., 0.01 → 0.1) and add dropout.

2. **Clean baseline comparison:** Run `eval_codebleu.py` with `GEN_FILE = "alp14_generated.rs"`
   (pre-fmt baseline) against **unformatted** gold to recover the true pre-change baseline.

3. **Diagnose worst commands:** Inspect `RSI_PLANE_SYSREG_READ` (0.183) and `PSCI_FEATURES` (0.210)
   — check if they are structurally novel or if the preamble context is missing key types.
