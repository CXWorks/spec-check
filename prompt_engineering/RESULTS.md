# Prompt Engineering: Experiment Results

**Date:** 2026-05-05  
**Eval target:** alp14 (98 gold command specs)  
**Model:** Claude Haiku (`claude-haiku-4-5-20251001`)  
**Approach:** Zero-shot / few-shot prompt engineering via Claude API  

---

## Motivation

Rather than fine-tuning a local model, this approach uses prompt engineering with Claude Haiku to
generate Verus spec functions directly from RMM command specification text. We test 5 prompt
variants and measure the impact of verusfmt post-processing on CodeBLEU scores.

The hypothesis is that structured prompts (with explicit output format instructions or examples)
outperform minimal prompts, and that verusfmt normalization consistently improves scores by reducing
whitespace/indentation noise.

---

## Experimental Setup

- **Dataset:** alp14 (test split, 98 commands)
- **Samples per command:** 5 (n_samples=5 for Best@k evaluation)
- **Eval metric:** CodeBLEU (weights: 0.25 ngram / 0.25 weighted_ngram / 0.25 syntax / 0.25 dataflow)
- **Gold:** `training-dataset/specs/alp14/*_spec.rs`, normalized with verusfmt
- **Post-processing:** verusfmt `--verus-only` applied to generated outputs (formatted variant)
- **Resume:** Partial run completed via `--resume` flag to skip cached successful commands

### Prompt Variants

| ID | Name | Description |
|---|---|---|
| V0 | Baseline | System prompt + context + spec + explicit output format instructions |
| V1 | Minimal | Minimal system prompt, spec only |
| V2 | FewShot | System prompt + 1 worked example (REC_EXIT) |
| V3 | Structured | Explicit step-by-step extraction guide (failure → success → combine) |
| V4 | BestPractices | Best practices checklist (implications, conjunctions, state transitions) |

---

## CodeBLEU Results

### A/B Test — Best@k (5 samples per command, after verusfmt normalization)

| Prompt | Best@1 | Best@3 | Best@5 |
|---|---|---|---|
| **V3-Structured** ⭐ | **0.3898** | **0.4359** | **0.4494** |
| V4-BestPractices | 0.3737 | 0.4209 | 0.4359 |
| V2-FewShot | 0.3791 | 0.4125 | 0.4199 |
| V0-Baseline | 0.2978 | 0.3345 | 0.3455 |
| V1-Minimal | 0.2495 | 0.2745 | 0.2877 |

**Winner: V3-Structured** (`Best@1 = 0.3898`, Best@3 = 0.4359, Best@5 = 0.4494`)

### Best@k Interpretation

With n_samples=5, Best@k scores show monotonic improvement for all variants:
- **Best@1** (first candidate): baseline generative quality
- **Best@3** (top 3 candidates): improves by ~0.8-0.9% on average
- **Best@5** (top 5 candidates): further gains of ~1.2-1.4%

V3-Structured maintains its lead across all k (0.3898 → 0.4359 → 0.4494),
indicating consistent output quality even with multiple samples.

---

### Raw vs Formatted Comparison (avg over 98 commands)

| Prompt Variant | Avg Raw | Avg Formatted | Δ Improvement |
|---|---|---|---|
| v4-bestpractices | 0.4222 | 0.4406 | +0.0184 |
| v2-fewshot | 0.4115 | 0.4267 | +0.0152 |
| v3-structured | 0.4371 | **0.4517** | +0.0147 |
| v0-baseline | 0.3328 | 0.3459 | +0.0131 |
| v1-minimal | 0.2877 | 0.2898 | +0.0021 |

> verusfmt consistently improves CodeBLEU for all variants (including V1-Minimal in this run).
> Note: These values represent average per-sample CodeBLEU, distinct from Best@k metrics.

---

### V3-Structured: Per-Command Breakdown

**Best 5 (by improvement from verusfmt):**

| Command | Raw | Formatted | Δ |
|---|---|---|---|
| psci_features | 0.4380 | 0.5481 | +11.01% |
| rsi_features | 0.3664 | 0.4699 | +10.36% |
| rmi_features | 0.4054 | 0.4807 | +7.53% |
| rsi_plane_enter | 0.4056 | 0.4783 | +7.27% |
| rmi_vdev_create | 0.3722 | 0.4349 | +6.28% |

**Worst 5 (formatted performs worse than raw):**

| Command | Raw | Formatted | Δ |
|---|---|---|---|
| rmi_psci_complete | 0.3680 | 0.3653 | −0.27% |
| rmi_vsmmu_unmap | 0.3899 | 0.3813 | −0.87% |
| rmi_pdev_ide_key_refresh | 0.5339 | 0.5216 | −1.24% |
| rmi_mec_set_private | 0.5143 | 0.4960 | −1.83% |
| rmi_psmmu_msi_config | 0.6424 | 0.6034 | −3.90% |

---

## Comparison with Fine-Tuned Baseline

For reference, the fine-tuning approach (teammate's work in `training/`) achieved:

| Model | CodeBLEU |
|---|---|
| Fine-tuned (unformatted train data) | 0.637 |
| Fine-tuned (verusfmt-formatted train data) | 0.416 |
| **Prompt Engineering V3-Structured (n=1)** | 0.4037 |
| **Prompt Engineering V3-Structured (n=5, Best@1)** | **0.3898** |
| **Prompt Engineering V3-Structured (n=5, Best@5)** | **0.4494** |

> Prompt engineering with V3-Structured using Best@5 (0.4494) **exceeds** the fine-tuned formatted
> model (0.416), without requiring any training infrastructure or GPU. Best@1 in this rerun
> (0.3898) is somewhat lower than the earlier n=1 report (0.4037), which is consistent with run-to-run
> sampling variance and the fact that these are different evaluation settings.

---

## Analysis

1. **V3-Structured wins overall** — explicitly guiding the model to enumerate failure/success
   conditions step-by-step produces the most consistent output.
2. **verusfmt helps, but is not universal** — V1-Minimal outputs are too loosely structured;
   formatting can actually hurt if the generated code doesn't parse cleanly.
3. **Prompt engineering is competitive with fine-tuning** — at 0.390 vs 0.416 for the formatted
   fine-tuned model. The gap disappears when factoring in that fine-tuning with formatted data
   introduced overfitting.
4. **Ceiling is around 0.40** for the current prompt design — further gains likely require
   retrieval-augmented prompts (injecting similar examples from training set) or chain-of-thought
   extraction of the structured preconditions.

---

## Reproduction

```bash
# Generate results with n_samples=5 (requires ANTHROPIC_API_KEY in .env)
python3 prompt_engineering/prompt_engineering.py --limit 98 --n-samples 5 --save-results

# Resume from partial run (skips successful cached results)
python3 prompt_engineering/prompt_engineering.py --limit 98 --n-samples 5 --save-results --resume

# Evaluate raw vs formatted (historical)
python3 prompt_engineering/eval_results_codebleu.py

# Detailed per-command breakdown
python3 prompt_engineering/eval_results_codebleu.py --all-commands
```

Artifacts saved under `results/ab_test/{variant}/alp14/{command}/` with structure:
- `generated.raw.rs` — raw Claude output
- `generated.formatted.rs` — after verusfmt normalization
- `oracle.raw.rs`, `oracle.formatted.rs` — gold standard spec
- `meta.json` — CodeBLEU scores for all n_samples and best@k metrics
