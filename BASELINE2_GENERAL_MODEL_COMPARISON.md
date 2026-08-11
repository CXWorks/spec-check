# Baseline 2: Comparison Against General Models/Agents (Claude 4.5 Haiku, GPT)

**Goal** compare
**our pipeline** (fine-tuned, LoRA-tuned Qwen 4B + repair loop) against
**general-purpose models prompted directly** (no fine-tuning, just prompt
engineering) on the same task — generating Verus specs for the alp14 command
set (98 commands) — and check both output quality (CodeBLEU) and actual
correctness (Verus compile/verify pass rate). Two general models have been
run so far: **Claude 4.5 Haiku** (Iterations 1–5) and **GPT (`gpt-5.6-sol`)**
(this session, see below).

This doc doesn't add a new experiment; it organizes work that already exists.
The general-model side was run across
[`prompt_engineering/RESULTS_V3.md`](prompt_engineering/RESULTS_V3.md)
Iterations 1–5, using **Claude 4.5 Haiku** as the generator (see
`ClaudeHaikuModel` in `prompt_engineering/prompt_engineering_v3.py:161`) with
the iteratively-tuned V3 prompt (RAG context, few-shot example, output
self-check rules, family-scoped guidance, hallucination-suppression rules).
No fine-tuning was involved on this side — all of the improvement documented
below came purely from prompt engineering against a general model's frozen
weights.

## Setup: what's actually comparable

All three were evaluated on the same benchmark: alp14, 98 commands, CodeBLEU
against oracle + real Verus verification (not just similarity).

| | General model 1 (Claude 4.5 Haiku) | General model 2 (GPT `gpt-5.6-sol`) | Our pipeline (fine-tuned Qwen) |
|---|---|---|---|
| Generator | Claude 4.5 Haiku via API, zero-shot/few-shot prompting | GPT `gpt-5.6-sol` via OpenAI API, zero-shot/few-shot prompting | `item_split_v4_best` — Qwen 4B, LoRA fine-tuned on alp14-style train split |
| Adaptation method | Prompt engineering only (5 iterations) | Same prompt as Claude (preamble restored, see below); no GPT-specific tuning | Fine-tuning on train split + prompt alignment + Verus-feedback repair loop |
| Result artifacts | `results/ab_test/v3/alp14/`, `results/ab_test/v3/alp14_verus_check_summary.json` | `results/ab_test_gpt/v3_gpt/alp14/`, `results/ab_test_gpt/v3_gpt/alp14_verus_check_summary.json` | `results/ab_test_qwen_v3retrained/`, `results/ab_test_qwen_v4/` |
| Source doc | `prompt_engineering/RESULTS_V3.md` Iterations 1–5 | This session (see below) | `prompt_engineering/RESULTS_V3.md` Iterations 6–7 |

## General model (Claude 4.5 Haiku): Iterations 1–5 summary

All numbers below are from `prompt_engineering/RESULTS_V3.md`; re-verified
against the saved artifacts in `results/ab_test/v3/` for this doc.

| Iteration | Change | CodeBLEU Best@1 / Best@3 / Best@5 | Verus pass rate |
|---|---|---|---|
| 1 (baseline V3) | Few-shot example + hard format constraints + self-check | 0.3851 / 0.4177 / 0.4303 | not measured |
| 2 (semantic-first) | Replaced hard-format pressure with semantic-priority policy; family-scoped RTT/VDEV/DATA guidance | 0.3597 / 0.3967 / 0.4138 (regressed further before the fix below) | not measured |
| 2 (scoped fix) | Scoped the RTT/VDEV/DATA rules to only apply to those families instead of globally | 0.4398 / 0.4709 / 0.4811 | not measured |
| 3 | Forbade hallucinated symbols (`UInt(...)` as a function, invented `RMI_SUCCESS`/`RMI_OK`/`RSI_OK` variants) | 0.4284 / 0.4745 / 0.4843 | not measured |
| 4 | (validation run of Iteration 3's prompt) | 0.4415 avg | **92/98 (93.88%)** |
| 5 | Added "no namespace prefixes on constants" rule (e.g. `RmmPas::PAS_NS` → `PAS_NS`) | **0.4437 / 0.4745 / 0.4845** | **95/98 (96.94%)** — re-verified: `results/ab_test/v3/alp14_verus_check_summary.json` → `{"pass": 95, "fail": 3, "pass_rate": 96.94}` |

Best/final general-model result (Iteration 5): **CodeBLEU Best@1 = 0.4437,
Verus pass rate = 95/98 (96.94%)**. The 3 remaining failures are all
`missing_symbol` (fabricated helper names), not systemic prompt bugs.

## General model 2 (GPT, `gpt-5.6-sol`): new run this session

New script: [`prompt_engineering/prompt_engineering_v3_gpt.py`](prompt_engineering/prompt_engineering_v3_gpt.py).
Same `PROMPT_V3_SYSTEM` as the Claude track (i.e. it already includes every
fix from Iterations 1–5 and 7's RMI/RSI result-type correction — nothing
GPT-specific was changed there), same RAG setup (`rag/index.json` +
`rag/index.npz`, top-k 3; the index had to be rebuilt from `rag/rules.jsonl`
because the on-disk `rag/index.pkl` was in a stale format `retriever.py` no
longer reads), `n_samples=5` with best-of-5 CodeBLEU selection — all identical
to the Claude run for comparability.

**One deliberate difference**: the shared Verus preamble/context, which
Iteration 6 removed from the template for the fine-tuned Qwen model (since
that model had the preamble baked into its training examples and didn't need
to see it again at inference), is **restored** in the GPT template. GPT has
never seen this project's preamble/DSL at all — without it, the model can
only guess at real struct/enum/helper names (signature/helper
symbols — would be invisible to it).

Command: `python3 prompt_engineering/prompt_engineering_v3_gpt.py --split test --limit 98 --n-samples 5 --rag-index rag/index.json --rag-top-k 3 --save-results`
Results: `results/ab_test_gpt/v3_gpt/alp14/`.

One operational issue hit during the run, fixed in the script: the initial
template accidentally included the RAG-retrieved-rules block twice (once
inside `{context}`, which `evaluate_prompt_variant` already appends
`retrieved_rules` onto, and once again via a separate `{retrieved_rules}`
placeholder in the template) — fixed by dropping the redundant placeholder
before the real run. Also, one command's generation hung for close to an hour
with no error (the underlying API call had no timeout) — added a 180s
per-request timeout with retry in `GPTModel.generate()`, and re-ran with
`--resume` so the run continued from the 66 commands already completed
instead of restarting from scratch.

### Results (CodeBLEU, 98/98 commands)

| Metric | Value |
|---|---|
| Best@1 | 0.5720 |
| Best@3 | 0.6014 |
| Best@5 | 0.6285 |

### Results (Verus, 98/98 commands)

Ran on the GPU server (`/mnt/md0/zhushan/spec-gen`, Verus
`0.2026.04.12.f1166c4`, same binary/version used for the Claude/Qwen checks
in `RESULTS_V3.md`) after copying `results/ab_test_gpt/` over — this local
Mac (arm64, no Rust/rustup toolchain) can't run Verus itself, and the
project's `verus-x86-linux` binary is Linux x86 anyway. Summary saved to
`results/ab_test_gpt/v3_gpt/alp14_verus_check_summary.json`.

| Metric | Value |
|---|---|
| Pass | 65/98 |
| Fail | 33/98 |
| **Pass rate** | **66.33%** |

Failure reasons: `type_mismatch` (19), `verus_error` (10), `parse_error` (4).
Unlike Claude's 3 remaining failures (all clean `missing_symbol`) or our
Qwen's mostly-`missing_symbol`/`type_mismatch` spread, GPT's failures skew
toward `type_mismatch` — plausible-looking Rust that doesn't actually
type-check against the preamble's real signatures, plus a few outright
`parse_error`s (malformed function items).

## Our pipeline (fine-tuned Qwen): comparable numbers

From the same `RESULTS_V3.md`, Iterations 6–7, and re-verified against
`results/ab_test_qwen_v4/v4_verus_check_summary_repaired_repaired.json`:

| Stage | CodeBLEU Best@1 | Verus pass rate |
|---|---|---|
| `item_split_v3_e2_best` (Iteration 6, before training-prompt fix) | 0.7627 | 15/98 (15.3%) |
| + 6 rounds of Verus-feedback self-repair | — | 42/98 (42.9%) |
| `item_split_v4_best` (Iteration 7, retrained on corrected prompt), fresh generation, no repair | **0.8389** | 35/98 (35.7%) |
| `item_split_v4_best` + 2 rounds of self-repair | — | **47/98 (48.0%)** — re-verified: `results/ab_test_qwen_v4/v4_verus_check_summary_repaired_repaired.json` → `{"pass": 47, "fail": 51, "pass_rate": 47.96}` |

## Head-to-head

| Metric | Claude 4.5 Haiku (prompted only) | GPT `gpt-5.6-sol` (prompted only, preamble restored) | Our fine-tuned Qwen (+ repair loop) |
|---|---|---|---|
| CodeBLEU Best@1 / Best@3 / Best@5 | 0.4437 / 0.4745 / 0.4845 | 0.5720 / 0.6014 / 0.6285 | **0.8389** / — / — |
| Verus pass rate | **95/98 (96.94%)** | 65/98 (66.33%) | 47/98 (48.0%) |

GPT lands in the middle on both metrics: higher CodeBLEU than Claude (0.5720
vs 0.4437 Best@1) but well behind it on actual Verus pass rate (66.33% vs
96.94%) — and still clearly ahead of our fine-tuned Qwen's 48.0% pass rate,
despite Qwen's much higher CodeBLEU (0.8389). So the CodeBLEU-vs-Verus
disconnect isn't just a Claude-vs-Qwen thing — it shows up again with a third,
independently-prompted model in the middle of both rankings, which is further
evidence CodeBLEU rank order and Verus-pass rank order genuinely don't agree
with each other across models, not just within one.

## Interpretation

This is a genuinely mixed, non-obvious result — worth stating plainly rather
than picking whichever metric favors "our" pipeline:

1. **On CodeBLEU, our fine-tuned model wins by a wide margin** (0.84 vs 0.44
   Claude vs 0.57 GPT). This makes sense: fine-tuning on the train split
   teaches the model to reproduce the oracle's exact style (naming, structure,
   type aliases), which is exactly what a text-similarity metric rewards —
   neither general model was ever trained on this oracle style.
2. **On actual correctness (Verus pass rate), both general models beat our
   fine-tuned pipeline**, and by very different margins: Claude 96.94% and GPT
   66.33%, vs our Qwen's 48.0%. This is the more important metric, since it's
   a real compiler/verifier check, not a similarity proxy — and it directly
   confirms the finding already flagged in Iteration 7 of `RESULTS_V3.md`:
   **CodeBLEU is a poor proxy for Verus-verifiable correctness.** All three
   models produce plausible-looking Rust; only checking it against the actual
   verifier reveals how differently "plausible" and "correct" track each
   other per model. GPT sitting in the *middle* of both rankings (better
   CodeBLEU than Claude, worse Verus pass rate; worse CodeBLEU than Qwen,
   better Verus pass rate) is itself useful evidence that the disconnect is a
   property of the metric, not an artifact of any one model pair.
3. **The Claude/GPT gap (96.94% vs 66.33%) is on the exact same prompt**,
   which narrows down what's driving it: both used the identical
   `PROMPT_V3_SYSTEM` (with all Iteration 1–5/7 anti-hallucination rules) and
   identical RAG setup — the only differences are the base model and (for GPT)
   the restored preamble block. So this specific gap is attributable to model
   capability/behavior differences on this task, not prompt quality
   differences. GPT's failures skew toward `type_mismatch` (19/33) rather than
   `missing_symbol` — it's inventing fewer nonexistent names than earlier
   Claude iterations did, but getting real preamble types wrong more often.
4. **Model capability, not just fine-tuning, appears to be a real factor for
   our pipeline's gap too.** Claude 4.5 Haiku and GPT are both substantially
   larger/more capable general models than the 4B LoRA-tuned Qwen checkpoint,
   and both beat it on Verus pass rate despite zero fine-tuning. This is
   consistent with Iteration 7's finding that our pipeline's remaining 51
   failures have "no single dominant pattern" — i.e. may partly reflect a
   capability ceiling for a 4B model under this repair strategy, not just a
   prompt/data problem.
5. **The comparison is not fully apples-to-apples on repair**: both
   general-model runs used `n_samples=5` with CodeBLEU-based best-of-5
   selection per command but no Verus-feedback repair loop; our pipeline used
   single-sample generation plus up to 2 rounds of actual Verus-error-feedback
   repair. A fairer follow-up would give the general models the same
   repair-loop treatment (see Not yet done).

## Bottom line

For this specific goal — generating Verus specs that actually **pass
verification** — both general-purpose models beat our fine-tuned,
repair-augmented Qwen pipeline on real Verus pass rate (Claude 96.94%, GPT
66.33%, vs our 48.0%), even though our pipeline wins decisively on CodeBLEU
similarity (0.8389 vs 0.4437 / 0.5720). The size of the gap varies a lot by
which general model is used — Claude's margin over us is huge, GPT's is more
modest — which suggests base model capability is a real, model-dependent
factor, not just "any general model beats fine-tuning here." This reinforces
that CodeBLEU should not be used alone to judge pipeline quality, and
suggests the fine-tuned pipeline's ceiling is at least partly a
model-capability issue, not only a prompt/training-data issue.

## Not yet done

- Run the same Verus-feedback self-repair loop (`repair_loop_verus.py`)
  against Claude's 3 and GPT's 33 remaining failures, to see whether either
  general model can close the gap further the same way it closed most of
  ours, and whether GPT's `type_mismatch`-heavy failures are as repairable as
  Claude's `missing_symbol` ones.
- Run our repair loop *using Claude or GPT as the repair judge* against our
  own Qwen model's 51 remaining failures (explicitly proposed as a next step
  in Iteration 7) — tests whether the gap is about repair-time judgment
  quality specifically, rather than base generation quality.
- Investigate GPT's 19 `type_mismatch` failures specifically — is there a
  single dominant pattern (like Claude's earlier `.is_Ok()`/`UInt()` bugs)
  that a targeted prompt rule could fix, the same way Iterations 3 and 5 fixed
  Claude's dominant failure modes?
- Cost/latency comparison (API cost + wall-clock for Claude vs GPT vs local
  GPU time for Qwen fine-tuning + inference) — not measured in any track so
  far. Worth noting operationally: GPT's run took noticeably longer in
  wall-clock time than Claude's (multi-hour for 490 generations, including
  one single-command hang before a timeout was added), which matters for
  practical pipeline choice even independent of quality.
