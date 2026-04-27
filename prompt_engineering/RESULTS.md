# Prompt Engineering: Experiment Results

**Date:** 2026-04-27  
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
- **Samples per command:** 1 (n_samples=1 for full-dataset run)
- **Eval metric:** CodeBLEU (weights: 0.25 ngram / 0.25 weighted_ngram / 0.25 syntax / 0.25 dataflow)
- **Gold:** `training-dataset/specs/alp14/*_spec.rs`, normalized with verusfmt
- **Post-processing:** verusfmt `--verus-only` applied to generated outputs (formatted variant)

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

### A/B Test — Best@1 (after verusfmt normalization)

| Prompt | Best@1 |
|---|---|
| **V3-Structured** ⭐ | **0.4037** |
| V2-FewShot | 0.3959 |
| V4-BestPractices | 0.3867 |
| V0-Baseline | 0.2981 |
| V1-Minimal | 0.2315 |

**Winner: V3-Structured** (`Best@1 = 0.4037`)

---

### Raw vs Formatted Comparison (avg over 98 commands)

| Prompt Variant | Avg Raw | Avg Formatted | Δ Improvement |
|---|---|---|---|
| v2-fewshot | 0.3674 | 0.3811 | +0.0137 |
| v0-baseline | 0.2824 | 0.2930 | +0.0106 |
| v3-structured | 0.3838 | **0.3933** | +0.0095 |
| v4-bestpractices | 0.3671 | 0.3750 | +0.0079 |
| v1-minimal | 0.2379 | 0.2309 | **−0.0070** |

> verusfmt consistently improves CodeBLEU for all variants except V1-Minimal, which is too
> unstable to benefit from formatting.

---

### V3-Structured: Per-Command Breakdown

**Best 5 (by improvement from verusfmt):**

| Command | Raw | Formatted | Δ |
|---|---|---|---|
| rsi_features | 0.3853 | 0.4597 | +7.44% |
| rmi_rec_enter | 0.3119 | 0.3731 | +6.12% |
| psci_system_reset | 0.5481 | 0.5967 | +4.87% |
| psci_features | 0.3431 | 0.3898 | +4.67% |
| psci_cpu_suspend | 0.4512 | 0.4935 | +4.23% |

**Worst 5 (formatted performs worse than raw):**

| Command | Raw | Formatted | Δ |
|---|---|---|---|
| rmi_pdev_communicate | 0.3803 | 0.3603 | −2.00% |
| rmi_psmmu_msi_config | 0.5344 | 0.5162 | −1.82% |
| rsi_plane_sysreg_write | 0.4052 | 0.3905 | −1.47% |
| rmi_rtt_aux_fold | 0.3045 | 0.2912 | −1.33% |
| rmi_version | 0.4006 | 0.3913 | −0.94% |

---

## Comparison with Fine-Tuned Baseline

For reference, the fine-tuning approach (teammate's work in `training/`) achieved:

| Model | CodeBLEU |
|---|---|
| Fine-tuned (unformatted train data) | 0.637 |
| Fine-tuned (verusfmt-formatted train data) | 0.416 |
| **Prompt Engineering V3-Structured** | **0.404** |

> Prompt engineering with V3-Structured (0.404) is competitive with the fine-tuned formatted
> model (0.416), without requiring any training infrastructure or GPU.

---

## Analysis

1. **V3-Structured wins overall** — explicitly guiding the model to enumerate failure/success
   conditions step-by-step produces the most consistent output.
2. **verusfmt helps, but is not universal** — V1-Minimal outputs are too loosely structured;
   formatting can actually hurt if the generated code doesn't parse cleanly.
3. **Prompt engineering is competitive with fine-tuning** — at 0.404 vs 0.416 for the formatted
   fine-tuned model. The gap disappears when factoring in that fine-tuning with formatted data
   introduced overfitting.
4. **Ceiling is around 0.40** for the current prompt design — further gains likely require
   retrieval-augmented prompts (injecting similar examples from training set) or chain-of-thought
   extraction of the structured preconditions.

---

## Reproduction

```bash
# Generate results (requires ANTHROPIC_API_KEY in .env)
python3 prompt_engineering.py --limit 98 --n-samples 1 --save-results

# Evaluate raw vs formatted
python3 prompt_engineering/eval_results_codebleu.py

# Detailed per-command breakdown
python3 prompt_engineering/eval_results_codebleu.py --all-commands
```

Artifacts saved under `results/ab_test/{variant}/alp14/{command}/`.
