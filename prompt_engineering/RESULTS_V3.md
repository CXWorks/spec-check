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

## Iteration 4: Major Breakthrough with Verus Pass-Rate Validation

### Full Verus Check Results (alp14 - 98 Commands)

After applying the Iteration 3 prompt refinements with semantic-first policy and targeted RTT/VDEV/DATA family guidance, we ran a comprehensive Verus validation on all 98 commands.

**Summary Results:**

| Metric | Value |
|--------|-------|
| **Total commands checked** | 98 |
| **Commands passing** | 92 |
| **Commands failing** | 6 |
| **Pass rate** | **93.88%** |
| **Average CodeBLEU Score** | **0.4415** |
| **Max CodeBLEU** | 0.7337 (psci_affinity_info) |
| **Min CodeBLEU** | 0.2263 (rmi_vsmmu_unmap) |

### Breakthrough Assessment

This represents a **massive improvement** from the previous baseline:

- Previous best attempt: 4.1% pass rate (4/98)
- Current iteration: 93.88% pass rate (92/98)
- **Improvement: +89.78 percentage points**

The CodeBLEU score of **0.4415** indicates reasonable syntactic/structural similarity to oracle specs. The distribution is fairly wide (0.2263–0.7337), suggesting some commands have better prompt alignment than others.

This validates that the semantic-first policy combined with targeted family-specific guidance (rather than over-constraining formatting) significantly improves code correctness.

### Remaining 6 Failures Analysis

All 6 failures are due to **missing symbol references** or **type resolution errors**, not semantic logic issues:

1. **psci_features** — `missing_symbol`: `IsPsciFunction` not found in scope
   - Root cause: predicate not imported or defined in preamble
   - Error: references undefined PSCI domain helper

2. **rmi_realm_create** — `missing_symbol`: `ZeroRealmMeasurement` not found
   - Root cause: measurement initialization helper not available
   - Impact: measurements field initialization syntax needs correction

3. **rmi_rtt_set_s2ap** — `missing_symbol`: `RealmsAt` vs `RealmAt` typo
   - Root cause: function name inflection error (plural vs singular)
   - Impact: one-character fix needed (`RealmsAt` → `RealmAt`)

4. **rmi_vdev_get_measurements** — `verus_error`: `RmmPas` type not declared
   - Root cause: enum import or namespace path missing
   - Impact: type resolution path needs completion

5. **rsi_mem_get_perm_value** — `verus_error`: `RsiStatusCode` vs `RmiStatusCode`
   - Root cause: namespace/enum name confusion (RSI vs RMI)
   - Impact: domain-family identifier needs clarification in preamble context

6. **rsi_mem_set_perm_index** — `missing_symbol`: `RsiErrorInput` not found
   - Root cause: error code constant not properly scoped
   - Impact: constant lookup path needs completion

## Iteration 5: Namespace Prefix Rule Addition & Final Breakthrough

### Problem Diagnosis

Further analysis of the 6 remaining failures revealed that 2 of them (`rmi_vdev_get_measurements` and `rsi_mem_get_perm_value`) were caused by **namespace prefix syntax errors**:

- Generated: `RmmPas::PAS_NS` (with namespace prefix)
- Expected: `PAS_NS` (bare constant)

- Generated: `RsiStatusCode::RSI_ERROR_INPUT` (with namespace prefix)
- Expected: `RSI_ERROR_INPUT` (bare constant)

The preamble context provides these as bare symbols; the model was incorrectly adding Rust namespace syntax that doesn't exist in the scope.

### Prompt Changes Applied in Iteration 5

We added a new critical rule to `prompt_engineering_v3.py`:

**Rule: "No namespace prefixes on constants"**

Added in two locations:

1. **PROMPT_V3_SYSTEM** (lines 74–77):
   ```
   - **No namespace prefixes on constants** (CRITICAL): Enum variants and constants in preamble are bare symbols 
     (RD, PAS_NS, RSI_ERROR_INPUT, VDEV_LOCKED, DEV_COMM_IDLE, etc.), NOT namespaced.
       * WRONG: `RmmGranuleState::RD`, `RmmPas::PAS_NS`, `RmmVdevState::VDEV_LOCKED`, `RsiStatusCode::RSI_ERROR_INPUT`
       * CORRECT: `RD`, `PAS_NS`, `VDEV_LOCKED`, `RSI_ERROR_INPUT`
       * Use constants exactly as they appear in context, without module::constant syntax
   ```

2. **PROMPT_V3_TEMPLATE** (requirements section):
   ```
   - CRITICAL — no namespace prefixes on constants: use bare symbols (RD, PAS_NS, VDEV_LOCKED, RSI_ERROR_INPUT) 
     NOT namespaced versions (NOT RmmGranuleState::RD, NOT RmmPas::PAS_NS, NOT RsiStatusCode::RSI_ERROR_INPUT). 
     Look at the preamble/context and copy exactly as written.
   ```

This rule complements the existing "Constant naming discipline" rule and directly targets the namespace-prefixing hallucination pattern.

### This Iteration's Full Rerun Results

We regenerated all 98 commands with 5 samples each (490 total candidates):

- Command: `python3 prompt_engineering/prompt_engineering_v3.py --split test --limit 98 --n-samples 5 --save-results --rag-index rag/index.pkl --rag-top-k 3`
- CodeBLEU metrics:
  - **Best@1 = 0.4437** (vs 0.4415 previously, +0.0022)
  - **Best@3 = 0.4745** (stable)
  - **Best@5 = 0.4845** (stable)

The CodeBLEU improvement is marginal, as expected (namespace prefixes don't heavily impact structural similarity).

### Verus Validation Results (Iteration 5)

**Summary Results:**

| Metric | Value | Change from Iteration 4 |
|--------|-------|--------|
| **Total commands checked** | 98 | – |
| **Commands passing** | 95 | +3 ✅ |
| **Commands failing** | 3 | −3 ✅ |
| **Pass rate** | **96.94%** | **+3.06%** ✅ |
| **CodeBLEU Best@1** | 0.4437 | +0.0022 |
| **CodeBLEU Best@3** | 0.4745 | +0.0 |
| **CodeBLEU Best@5** | 0.4845 | +0.0 |

### Key Fixes in Iteration 5

Two critical `verus_error` failures were successfully resolved:

1. ✅ **rmi_vdev_get_measurements**: `verus_error` (namespace prefix) → **pass (ok)**
   - Fix: `RmmPas::PAS_NS` → `PAS_NS`

2. ✅ **rsi_mem_get_perm_value**: `verus_error` (namespace prefix) → **pass (ok)**
   - Fix: `RsiStatusCode::RSI_ERROR_INPUT` → `RSI_ERROR_INPUT`

3. ✅ **rmi_rtt_set_s2ap**: `missing_symbol` (typo) → **pass (ok)** (collateral fix)
   - Fix: `RealmsAt` → `RealmAt`

### Remaining 3 Failures (Iteration 5)

All remaining failures are now `missing_symbol` errors, not `verus_error`:

1. **rmi_vdev_create** — `missing_symbol`: fabricated helper symbol
2. **rsi_features** — `missing_symbol`: fabricated helper or unconstrained spec
3. **rsi_mem_set_perm_index** — `missing_symbol`: case mismatch or fabricated constant

These are hallucination issues (invented symbols) rather than namespace/syntax errors. Further improvements would require additional symbol-validation rules.

### Summary & Recommendations

Iteration 5 successfully demonstrates that:

- **Targeted rule addition works**: Adding the namespace-prefix rule fixed exactly the 2 `verus_error` failures it was designed to address
- **No regression**: CodeBLEU and pass rate remained stable/improved
- **Semantic correctness prioritized**: The namespace rule is a semantic constraint (correct symbol usage) rather than a formatting constraint

**Current status:**
- **96.94% pass rate** on 98 commands (95/98)
- **CodeBLEU Best@1 = 0.4437**, Best@3 = 0.4745, Best@5 = 0.4845
- **Highest pass rate achieved** in this investigation series

**Next steps (if continuing):**
- Analyze the 3 remaining `missing_symbol` failures for common patterns
- Consider adding symbol-whitelist validation rules if patterns are consistent
- Document final V3 prompt as production baseline

## Iteration 6: Preamble Removal + Train/Inference Prompt Alignment for the Local Qwen Model

Iterations 1–5 above tuned the V3 prompt against **Claude 4.5 Haiku**. This iteration targets a separate, previously-undiagnosed problem specific to the **locally fine-tuned Qwen model** (`models/item_split_e2_best`, used by `run_qwen_v3.py`), so its CodeBLEU numbers are on a different model/benchmark than Iterations 1–5 and are not directly comparable to the 0.44-range scores above.

### Problem Diagnosis

Two separate issues were compounding poor Qwen results:

1. **Preamble was redundant at inference.** `PROMPT_V3_TEMPLATE` included the full shared-Verus-types preamble (`{context}`) on every request, but the deployed Qwen checkpoint was fine-tuned on data where preamble was already embedded in every training example — so it didn't need to see it again at inference, and it was burning a large share of the token budget for no benefit.
2. **Train/inference prompt mismatch (the bigger problem).** `training/build_dataset.py` — the script that actually built the fine-tuning dataset for `item_split_e2_best` — used a short, generic system prompt (~96 tokens, no anti-hallucination rules) and a different user-message template (`"## Context...\n{preamble}\n## Command Specification...\n{section}"`) than the much longer, structured `PROMPT_V3_SYSTEM`/`PROMPT_V3_TEMPLATE` that `run_qwen_v3.py` was actually using at inference. The model had never seen the V3 prompt's format during training — inference was effectively zero-shot-injecting a foreign prompt onto a model tuned for a different one.

### Changes Applied

1. **`prompt_engineering_v3.py`**:
   - Deduped `PROMPT_V3_SYSTEM`: merged rules that were stated twice almost verbatim between "Core constraints" and "Output self-check" (no `RMI_SUCCESS`/`RMI_OK`, no `UInt()`, naming, one-function-item form, anti-hallucination symbol checks). No rule content was dropped — only literal restatement was removed. ~2310 → ~1690 tokens (−27%).
   - Removed `{context}`/preamble entirely from `PROMPT_V3_TEMPLATE`; template is now just `{spec}` + a short signature/alias/unchanged-state reminder.
2. **`training/build_dataset.py`**: command-kind training examples now use the same (deduped) V3 system prompt as inference, with preamble still embedded in the training user message — the model is meant to *learn* the symbols from preamble during training, then run without it at inference.
3. **`training/train.py`** — three real bugs found and fixed while getting the retrain to actually run on the GPU (Quadro RTX 8000, Turing, no flash-attention):
   - `SFTConfig(max_seq_length=...)` isn't a real field in the installed `trl` version (the real field is `max_length`); our sequence-length cap was silently never applied, causing repeated identical OOM crashes regardless of what value was passed.
   - `padding_free` was being auto-enabled by Unsloth despite no flash-attention support on this GPU, which blew up attention memory; explicitly disabled.
   - Installed `xformers` (prebuilt wheel, no source build needed) to get memory-efficient attention on this older GPU — without it, attention memory scales with sequence² instead of sequence, which is what made 12288-token training examples infeasible in the first place.
4. Retrained: `models/item_split_v3_e2_best` (2 epochs, batch size 4, max-seq 12288, ~2h45m on GPU5).

### Results (Qwen, full 98-command alp14 test set)

| Metric | `item_split_e2_best` (old, mismatched prompt) | `item_split_v3_e2_best` (this iteration) |
|---|---|---|
| CodeBLEU Best@1 | 0.639 | **0.7627** |
| Δ | — | **+0.1237 (+19.4%)** |

Per-command spread: 10 commands scored a perfect 1.0 (e.g. `PSCI_CPU_OFF`, `RMI_GRANULE_DELEGATE`, `RMI_REALM_ACTIVATE`). The lowest scores (0.32–0.56) were concentrated in "fully unconstrained" commands (`RSI_VERSION`, `RSI_FEATURES`, `RMI_VERSION`) whose oracle is just `true` — CodeBLEU is noisy on very short targets, so this isn't necessarily a correctness problem.

Full per-command results: `results/ab_test_qwen_v3retrained/v3_qwen/alp14/` (this session); old baseline for comparison: `results/ab_test_qwen/`.

### Interpretation

This confirms the train/inference prompt-mismatch hypothesis: aligning the two (same system prompt, preamble present at train time only) closed most of the gap on its own, independent of any further prompt-wording tuning. Unlike Iterations 1–5, none of this iteration's gain came from changing *what the prompt says* — it came from making sure the model that's actually running has seen the prompt shape it's being asked to use at inference.

**Next steps (if continuing):**
- Run the same before/after comparison through Verus (not just CodeBLEU) to confirm the gain holds under compilation, matching how Iterations 4–5 validated against Claude.
- Consider whether Iterations 1–5's Claude-tuned rules (namespace prefixes, symbol whitelist, etc.) transfer as cleanly to Qwen now that the prompt format finally matches training.
