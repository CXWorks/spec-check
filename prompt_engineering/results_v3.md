# V3 Prompt Investigation Report

## Overview

This report summarizes a full comparison between `oracle` and `generated` outputs under the V3 prompt setup (`results/ab_test/v3/alp14`) and documents the prompt-tuning changes we introduced to improve output consistency.

## Full-Scan Findings (V3: Oracle vs Generated)

The following issue counts come from the full dataset scan:

- **family_mismatch** (cross-family symbol mix-up): **13**
- **state_param_order_diff** (`old_s` / `new_s` position mismatch): **84**
- **result_type_diff** (`result` type mismatch): **33**
- **bits_uint_type_diff** (`u64` / `u32` vs `Bits64` / `UInt64` / `UInt32`): **99**
- **fn_name_case_diff** (function naming style mismatch, uppercase vs snake_case): **98**

## What These Mismatches Mean

### 1) Family mismatch
Generated specs sometimes mixed symbol families that should remain domain-consistent, e.g.:

- using `vdev` symbols inside `pdev` commands
- using `psmmu` / `vsmmu` symbols interchangeably

This creates semantic drift and directly hurts matching quality.

### 2) `old_s` / `new_s` order mismatch
Many generated signatures place `old_s` and `new_s` in positions that differ from the oracle pattern (often oracle places them near the end).

### 3) `result` type mismatch
In multiple commands, generated code used a different `result` type than oracle expectations (e.g., `RmiCommandReturnCode` vs `Result<(), ...>` style).

### 4) Numeric type alias mismatch
Generated code frequently used primitive Rust numeric types (`u64`, `u32`) where oracle uses project aliases (`Bits64`, `UInt64`, `UInt32`).

### 5) Function name style mismatch
Generated function names were often uppercase command-style (e.g., `RMI_..._spec`) while oracle consistently uses lowercase snake_case (e.g., `rmi_..._spec`).

## V3 Prompt Tuning Changes We Added

To reduce these recurring errors, we upgraded `prompt_engineering_v3.py` in several ways:

### A) Borrowed from V2: Added a worked example
We added a few-shot style template example (`REC_EXIT`) to show the target implication structure:

- failure conditions as implications to error results
- success condition as implication to success postconditions

This provides concrete output shape guidance, not just abstract instructions.

### B) Strengthened hard constraints
We added stricter constraints in V3 prompt text, including:

- function must be lowercase snake_case: `{cmd_name_lower}_spec`
- explicitly use `result` parameter (no `old_s.result` / `new_s.result`)
- keep predicate/function arity aligned with provided context

### C) Added output self-check rules
Based on the full-scan findings, we introduced an explicit "Output self-check" block requiring the model to verify before final output:

1. **Family consistency** (no pdev/vdev or psmmu/vsmmu cross-mixing)
2. **Signature order consistency** (match established style; place `old_s/new_s` at end when that pattern is used)
3. **Type alias consistency** (prefer `Bits64` / `UInt64` / `UInt32` over raw primitives when aliases are available)
4. **Naming consistency** (exact lowercase snake_case function name)

## Follow-up Run: Observed Regression and Diagnosis

After introducing the stronger constraints and self-check guidance, we ran a full V3 evaluation:

- Command: `python3 prompt_engineering/prompt_engineering_v3.py --limit 98 --n-samples 5 --save-results`
- Result snapshot:
  - **Best@1 = 0.3851**
  - **Best@3 = 0.4177**
  - **Best@5 = 0.4303**

This was lower than the previous V3 backup baseline, so we performed a differential diagnosis against the retained backup artifacts.

### Differential Findings vs Backup

- **58 commands dropped**, **40 commands improved**
- No mass failure artifacts (no broad "all-zero" collapse)
- Degradation concentrated in more complex command families (notably `pdev` / `vdev` / `rtt` / `psmmu`)

Representative drops:

- `rmi_psmmu_msi_config`: −0.1887
- `rmi_pdev_p2p_disconnect`: −0.1800
- `rmi_vdev_map`: −0.1770

Representative gains:

- `psci_cpu_off`: +0.3465
- `rsi_host_call`: +0.2137
- `rsi_measurement_extend`: +0.2080

Interpretation: stronger formatting/style constraints improved some simpler commands, but likely over-constrained semantic choices in complex families.

## Iteration 2: V3 "Semantic-First" Refinement

To address the regression pattern, we updated `prompt_engineering_v3.py` again with a semantic-first policy.

### What Changed

1. Replaced "hard-format emphasis" with **semantic-priority policy**:
	- semantic correctness > cosmetic formatting
	- avoid inventing behavior beyond context/spec
2. Kept critical correctness constraints (result usage, arity consistency, implication discipline), but softened purely cosmetic pressure.
3. Upgraded self-check to include **domain symbol anchoring** for complex families:
	- for `pdev` / `vdev` / `rtt` / `psmmu`, prefer helper/accessor names already present in context
	- avoid analogy-based renaming across families
4. Template requirements now explicitly state:
	- prefer aliases when available, **but not at the expense of semantic correctness**
	- prioritize correct helper/function selection for complex families

## Quick Validation After Refinement

We executed a small smoke test after the semantic-first update:

- Command: `python3 prompt_engineering/prompt_engineering_v3.py --limit 5 --n-samples 1`
- Output confirmed the updated prompt text is active
- Smoke metrics:
  - **Best@1 = 0.6009**
  - **Best@3 = 0.6009**
  - **Best@5 = 0.6009**

This smoke run is not directly comparable to full evaluation, but it verifies:

- the revised prompt is being used
- the pipeline runs cleanly end-to-end after changes

## Iteration 2 Addendum: Targeted Fixes for Very Low-Score Commands

As part of Iteration 2 (semantic-first refinement), we further inspected the hardest commands (especially those with Best@1 around 0.1–0.2 and/or very low Best@5) and added targeted guidance.

### Commands Selected for Deep Inspection

Representative low-score set included:

- `rmi_rtt_aux_fold`
- `rmi_rtt_aux_destroy`
- `rmi_rtt_aux_create`
- `rmi_vdev_map`
- `rmi_rtt_unmap_unprotected`
- `rmi_data_destroy`

### Root Causes Observed (Oracle vs Generated)

1. **Signature incompleteness / implicit operand substitution**
	- Generated specs occasionally omitted explicit command operands and relied on hidden state fields (e.g., `old_s.cmd_input_x*`) where oracle used explicit parameters.

2. **Invalid state-transition formulation**
	- In some cases, postconditions compared `new_s` against itself (e.g., `X(new_s) == X(new_s) + delta`) instead of writing an `old_s -> new_s` transition.

3. **RTT error-branch payload inconsistency**
	- `RMI_ERROR_RTT` / `RMI_ERROR_RTT_AUX` level payloads were sometimes derived inconsistently from walk conditions, mixing old/new references in unstable ways.

4. **Map/Unmap coherence gaps**
	- Entry-state transitions and granule-state transitions were not always paired coherently, and error-path unchanged-state framing was sometimes missing.

### Prompt Updates Added for This Iteration

We added a dedicated targeted-prescriptions block in `prompt_engineering_v3.py` for RTT/VDEV/DATA families:

- enforce **signature completeness first** for command operands (`rd/ipa/level/index/rtt/top/data/vdev_ptr/addr`)
- disallow replacing missing operands with hidden state fields unless explicitly required by context/spec style
- enforce **state-transition sanity** (`old_s -> new_s`, never `new_s -> new_s` self-comparison updates)
- require consistent derivation of RTT error payloads from the triggering walk condition
- require map/unmap postcondition coherence and unchanged-state framing on error paths

These additions are intended to specifically raise floor performance on the persistent low-score tail while preserving the semantic-first policy introduced in Iteration 2.

## Latest Observation: Further Regression After Addendum

A subsequent full run showed a further drop:

- **Best@1 = 0.3597**
- **Best@3 = 0.3967**
- **Best@5 = 0.4138**

Compared with the retained backup baseline (`v3_backup_20260508_221621`), this is still lower:

- Best@1: **-0.0301**
- Best@3: **-0.0391**
- Best@5: **-0.0356**

Per-command comparison also showed broad degradation (**64 drops / 34 gains**), with notably large drops in several PSCI commands (e.g., `psci_features`, `psci_system_reset`, `psci_cpu_suspend`).

### Interpretation

The RTT/VDEV/DATA-focused addendum likely improved guidance for targeted RMI tails, but applying these constraints globally appears to introduce collateral pressure on unrelated families (especially PSCI/RSI style outputs).

### Prompt Adjustment Applied

To reduce collateral damage, we updated `prompt_engineering_v3.py` to explicitly scope RTT/VDEV/DATA targeted prescriptions:

- apply those constraints **only** when command semantics match RTT/VDEV/DATA families
- do **not** force those rules onto unrelated families (e.g., PSCI/RSI)

This keeps the targeted guidance for hard RMI commands while reducing over-constraint for other command groups.

## Recommended Next Evaluation Step

Run a medium/full comparable evaluation to validate recovery trend:

- Medium check: `--limit 30 --n-samples 5 --save-results`
- Full check: `--limit 98 --n-samples 5 --save-results`

Then compare per-command deltas vs the retained backup baseline to verify whether complex-family regressions are reduced.

## Latest Full Evaluation Update (Scoped RTT/VDEV/DATA Rules)

We ran the full comparable setting again after scoping RTT/VDEV/DATA targeted prescriptions to only relevant command families.

- Command: `python3 prompt_engineering/prompt_engineering_v3.py --limit 98 --n-samples 5 --save-results`
- Model: Claude 4.5 Haiku
- Prompt: V3-Structured

### Result Snapshot

- **Best@1 = 0.4398**
- **Best@3 = 0.4709**
- **Best@5 = 0.4811**

### Comparison to Previous Iteration (Post-Addendum Regression Run)

Previous run:

- Best@1 = 0.3597
- Best@3 = 0.3967
- Best@5 = 0.4138

Current deltas:

- **Best@1: +0.0801**
- **Best@3: +0.0742**
- **Best@5: +0.0673**

### Interpretation

This is a strong recovery and supports the hypothesis that RTT/VDEV/DATA-targeted constraints should be family-scoped rather than globally enforced. The update appears to reduce collateral damage on unrelated families while preserving guidance for difficult RMI tails.

In short, the scoped policy is currently the best-performing V3 configuration recorded in this report series so far.

## Iteration 3: Hallucinated Symbol Elimination (missing_symbol fix) + Full Rerun Outcome

In this iteration, we combined a targeted prompt fix for `missing_symbol` failures with a full comparable rerun on `results/ab_test/v3/alp14`.

### Root-Cause Findings (Why Iteration 3 Was Needed)

From Verus failure analysis (`alp14_verus_check_summary_latest.json`), 55/98 commands previously failed with `missing_symbol`. Two hallucination patterns dominated:

| Hallucinated symbol | Occurrences | Root cause |
|---|---|---|
| `UInt(x)` | 40 | `UInt`, `UInt64`, `UInt32` are type aliases, not callable functions. |
| `RMI_SUCCESS` / `RMI_OK` / `RSI_OK` | 32 | `RmiStatusCode` has error variants only; success should be `result.is_Ok()`. |

Together, these accounted for roughly 65% of `missing_symbol` failures.

### Prompt Changes Applied in Iteration 3

We added 6 targeted rules to `prompt_engineering_v3.py` in three places (system constraints, output self-check, and template requirements):

1. Forbid invented success variants (`RMI_SUCCESS` / `RMI_OK` / `RSI_SUCCESS` / `RSI_OK`), and require `result.is_Ok()` for success.
2. Forbid `UInt(...)` as a cast/function and require direct integer/bounds expressions.

This was designed to directly suppress the two highest-frequency hallucination classes without reintroducing heavy formatting pressure.

### This Session’s Full Rerun Results (with Iteration 3 changes)

We reran the full comparable setting in this session:

- Run setting: V3 full run (`--limit 98 --n-samples 5 --save-results`, RAG enabled)
- Log: `logs/v3_full_rerun.log`
- Final sampled-best metrics:
	- **Best@1 = 0.4284**
	- **Best@3 = 0.4745**
	- **Best@5 = 0.4843**

Aggregate summary from `python3 prompt_engineering/eval_results_codebleu.py`:

- **v3 Avg Raw = 0.4821**
- **v3 Avg Formatted = 0.4824**
- **Formatting gain = +0.0004**

Reference points from the same table:

- `v3_backup_20260508_221621` Avg Formatted = **0.4440**
- `v4` Avg Formatted = **0.4323**

### Integrated Analysis

1. **Iteration 3 is competitive in this workspace snapshot**: v3 remains the highest formatted average among listed variants in the current rerun context.
2. **Sampling still helps**: Best@3/Best@5 remain meaningfully above Best@1, indicating useful candidate diversity persists.
3. **Main bottleneck is semantic, not formatting**: the formatting delta is nearly zero (+0.0004), so future gains should come from semantic correctness rather than stylistic constraints.
4. **Iteration-3 fix direction is consistent with failures**: the newly added rules target the exact dominant hallucination classes (`UInt(...)`, nonexistent success variants), aligning fix strategy with observed Verus error modes.

### Verus Pass-Rate Note

- Historical checkpoints before this integrated rerun were:
	- Before environment fix: **0/98** (vstd runtime missing)
	- After environment fix only: **4/98 (4.1%)**
- A fresh full Verus pass-rate measurement after the Iteration 3 rerun is still pending and should be run as the next validation step.

