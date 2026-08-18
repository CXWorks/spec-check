# Baseline 2: Comparison Against General Models/Agents (Claude, GPT)

**Goal** compare
**our pipeline** (fine-tuned, LoRA-tuned Qwen 4B + repair loop) against
**general-purpose models prompted directly** (no fine-tuning, just prompt
engineering) on the same task — generating Verus specs for the alp14 command
set (98 commands) — and check both output quality (CodeBLEU) and actual
correctness (Verus compile/verify pass rate). Two general models have been
run so far: **Claude** (Iterations 1–6 on Claude 4.5 Haiku, superseded by
Iteration 7 on Claude Opus 4.8 — see below) and **GPT (`gpt-5.6-sol`)**
(this session, see below).

The general-model side started as a synthesis of existing work
([`prompt_engineering/RESULTS_V3.md`](prompt_engineering/RESULTS_V3.md)
Iterations 1–5, using **Claude 4.5 Haiku** as the generator — see the
`ClaudeModel` class, then still named `ClaudeHaikuModel`, in
`prompt_engineering/prompt_engineering_v3.py` — with the iteratively-tuned V3
prompt: RAG context, few-shot example, output self-check rules,
family-scoped guidance, hallucination-suppression rules). This doc's own
Iteration 6 then found and fixed a real bug in that setup (Claude's prompt
never actually included the preamble/helper-signature context) and added a
Verus-feedback repair loop on top. **Iteration 7 (current)** swaps the
generator model from Claude 4.5 Haiku to **Claude Opus 4.8** (`effort=high`)
— same prompt, same repair loop — and its numbers now supersede Iteration
6's as "the" Claude result throughout this doc (Iterations 1–6 are kept
below as historical record of how the prompt/repair-loop setup was built).
No fine-tuning was involved on the Claude/GPT side — all improvement
documented below came from prompt engineering plus a repair loop against a
general model's frozen weights.

## Setup: what's actually comparable

All three were evaluated on the same benchmark: alp14, 98 commands, CodeBLEU
against oracle + real Verus verification (not just similarity).

| | General model 1 (Claude Opus 4.8, `effort=high`) | General model 2 (GPT `gpt-5.6-sol`) | Our pipeline (fine-tuned Qwen) |
|---|---|---|---|
| Generator | Claude Opus 4.8 via API, zero-shot/few-shot prompting (Iterations 1–6 used Claude 4.5 Haiku on the same prompt; superseded, see Iteration 7) | GPT `gpt-5.6-sol` via OpenAI API, zero-shot/few-shot prompting | `item_split_v4_best` — Qwen 4B, LoRA fine-tuned on alp14-style train split |
| Adaptation method | Prompt engineering (5 iterations) + preamble restored + Verus-feedback repair loop (Iteration 6, model swapped to Opus 4.8 in Iteration 7) | Same prompt as Claude (preamble restored, see below); no GPT-specific tuning, no repair loop | Fine-tuning on train split + prompt alignment + Verus-feedback repair loop |
| Result artifacts | `results/ab_test/v3/alp14/`, `results/ab_test/v3/alp14_verus_check_summary.json`, `results/ab_test/v3/alp14_verus_check_summary_repaired.json` | `results/ab_test_gpt/v3_gpt/alp14/`, `results/ab_test_gpt/v3_gpt/alp14_verus_check_summary.json` | `results/ab_test_qwen_v3retrained/`, `results/ab_test_qwen_v4/` |
| Source doc | `prompt_engineering/RESULTS_V3.md` Iterations 1–5, this doc's Iterations 6–7 | This session (see below) | `prompt_engineering/RESULTS_V3.md` Iterations 6–7 |

## General model (Claude 4.5 Haiku): Iterations 1–5 summary (historical — superseded by Iteration 7)

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

## Iteration 6 (Claude 4.5 Haiku, historical — superseded by Iteration 7): restoring the preamble the GPT track already had

**Bug found:** `prompt_engineering_v3.py`'s `PROMPT_V3_TEMPLATE` (the Claude
track) never actually included `{context}` (the preamble: struct/enum/helper
signatures) — the GPT script's docstring explains that Iteration 6 of the
*Qwen* track dropped the preamble from the shared `V3_PROMPT` template
because the fine-tuned Qwen model had it baked into training data. That
reasoning doesn't apply to Claude (never fine-tuned on this preamble, same as
GPT) — GPT's script was written to restore it, but Claude's script was never
fixed the same way. So the 96.94% Iteration-5 number above was achieved
**blind**, with Claude never seeing a single real struct/enum/helper
signature — it was inferring plausible names/shapes purely from the spec
text, the RAG-retrieved rule snippets, and the one worked example in the
system prompt.

Fixed by restoring the same `Context: {context}` block GPT's template
already uses (verbatim match, replacing the old `{retrieved_rules}`-only
template to avoid double-inserting the RAG block). Same `PROMPT_V3_SYSTEM`,
unchanged.

**Result — a large regression, not an improvement:**

| Run | CodeBLEU Best@1 | Verus pass rate |
|---|---|---|
| Iteration 5 (blind, no preamble) | 0.4437 | 95/98 (96.94%) |
| Iteration 6a (preamble restored, same prompt otherwise) | 0.4995 | **24/98 (24.49%)** |

CodeBLEU went up (more real symbol names → closer textual match to oracle)
but Verus-verifiable correctness collapsed. Root-caused by direct inspection
of the failures: seeing the real preamble made Claude start actually
*calling* real helper functions it had previously avoided or simplified to
`true`, and it dropped the leading `s: S` state parameter in **37 of 74
failures (50%)** — e.g. `AddrIsGranuleAligned(pdev_ptr)` instead of
`AddrIsGranuleAligned(old_s, pdev_ptr)`. `PROMPT_V3_SYSTEM` already warned
about this class in passing ("helpers... take a leading `s: S` argument that
is easy to drop"), but the warning didn't hold up once the model was
confidently using real names everywhere instead of defaulting to `true`. The
remaining failures were the same pre-existing pitfall classes (wrong arg
types, `old_s.ImplFeatures()` dot-call instead of `ImplFeatures(old_s)`,
inventing `RMI_SUCCESS`) — also triggered more often for the same reason.

Added a dedicated `CRITICAL` rule to `PROMPT_V3_SYSTEM` with concrete
WRONG/CORRECT examples targeting exactly this pattern, and reran:

| Run | CodeBLEU Best@1 | Verus pass rate |
|---|---|---|
| Iteration 6b (+ explicit leading-`s: S` rule) | 0.5289 | **40/98 (40.82%)** |

The targeted fix worked on its target class (leading-`s`-drop failures fell
from 50% to 21% of the remaining failures) but exposed the *next* systemic
pattern underneath: several `RMI_ERROR_*`/`RSI_ERROR_*` enum variants are
declared in the real preamble as payload-carrying constructors (e.g.
`RMI_ERROR_RTT_AUX(int)`), and Claude was calling them as bare unit variants
(`ResultEqual(result, RMI_ERROR_RTT_AUX)` instead of
`RMI_ERROR_RTT_AUX(level)`). Same lesson `RESULTS_V3.md` Iteration 7/8
already documented for Qwen: one prompt rule reliably fixes its own class,
then the next class dominates — this stopped being a two-line prompt patch
and started being whack-a-mole across independent bug classes.

**Instead of continuing to prompt-patch bug classes one at a time, ran the
existing Verus-feedback self-repair loop** (`repair_loop_verus.py`, already
built for Qwen) against Claude's 58 Iteration-6b failures. New script
[`repair_loop_verus_claude.py`](repair_loop_verus_claude.py) reuses the same
`FIX_INSTRUCTIONS`, symbol-snippet extraction (pulls the real preamble
definition of any identifier named in a Verus error — this alone grounds the
payload-arity fix, since the error message names `RMI_ERROR_RTT_AUX` and the
snippet shows its real `(int)` signature), and retry-with-ruled-out-history
logic as the Qwen version, swapped to `ClaudeModel` (then still named
`ClaudeHaikuModel`) and — unlike the
Qwen repair loop, which passes an empty `context` because Qwen has the
preamble baked into training — passes the real `sample.preamble` through,
since Claude needs it. Ran with `--max-retries 10`:

| Run | CodeBLEU Best@1 | Verus pass rate |
|---|---|---|
| Iteration 6c (+ Verus-feedback repair loop, ≤10 retries/command) | **0.5410** | **95/98 (96.94%)** |

**55 of 58 failing commands resolved** (2–10 attempts each, only 2 commands
needed more than 5). Final pass rate lands back at exactly Iteration 5's
96.94% — but now achieved with the preamble genuinely in-context (an
apples-to-apples setup with GPT's track) and with CodeBLEU meaningfully
higher (0.5410 vs Iteration 5's blind 0.4437), since the repaired code now
uses real symbol names throughout instead of `true`-stub avoidance. The 3
remaining failures (`rmi_pdev_create`, `rsi_mem_set_perm_value`,
`rsi_vdev_get_info`) are three unrelated one-off bugs (a `!` applied to a
non-bool `RmmFeature` enum field, a parenthesization/parse error, and an
incompatible-type `==` comparison) — no shared root cause, unlike the
systemic classes above.

**Takeaway:** for a general (never fine-tuned) model, showing it the real
preamble is a net negative *before* a repair loop (regresses 96.94% →
24.49%) and roughly neutral-to-positive *after* one (96.94% → 96.94% with
higher CodeBLEU) — the preamble mainly shifts failures from "invents a
name" (undetectable-looking, but ~0% chance of ever passing Verus without
massive luck) to "gets a real symbol's arity/payload slightly wrong"
(concrete, machine-diagnosable, and exactly what the repair loop's
error-grounded retry is good at). Blind prompting only looked better
because Iteration 5 never spent a real-error-feedback repair budget on its
own 3 failures — comparing "blind + no repair" against "sighted + repair"
was never apples-to-apples in the first place. Result artifacts:
`results/ab_test/v3/alp14/` (overwritten in place),
`results/ab_test/v3/alp14_verus_check_summary.json` (pre-repair, 40/98),
`results/ab_test/v3/alp14_verus_check_summary_repaired.json` (post-repair,
95/98).

## Iteration 7 (Claude): switching the generator from Haiku to Opus 4.8 — current headline result

Same `PROMPT_V3_SYSTEM`/`PROMPT_V3_TEMPLATE`, same RAG setup, same repair
loop (`repair_loop_verus_claude.py`) as Iteration 6 — the only change is the
generator model: `claude-haiku-4-5-20251001` → `claude-opus-4-8` with
`thinking: {type: "adaptive"}` and `output_config: {effort: "high"}`. The
`ClaudeModel` class (renamed from `ClaudeHaikuModel` to reflect that it's no
longer Haiku-specific) and `prompt_engineering_v3.py`'s new `--model`/
`--effort` CLI flags were added to support this. Full run:
`python3 prompt_engineering/prompt_engineering_v3.py --split test --limit 98
--n-samples 5 --rag-index rag/index.json --rag-top-k 3 --save-results
--model claude-opus-4-8 --effort high` (overwrote `results/ab_test/v3/`
in place).

**Fresh generation, before repair — a regression vs. Haiku, despite higher CodeBLEU:**

| Run | CodeBLEU Best@1/3/5 | Verus pass rate |
|---|---|---|
| Iteration 6c (Haiku, post-repair, for reference) | 0.5410 | 95/98 (96.94%) |
| Iteration 7a (Opus 4.8, fresh generation, no repair) | 0.5664 / 0.5937 / 0.6051 | **47/98 (47.96%)** |

Despite scoring *higher* on CodeBLEU than even Haiku's post-repair number,
Opus's raw pass rate is roughly half of Haiku's Iteration-6c/Iteration-5
results. Inspecting failures showed Opus tripping the exact bug classes
`PROMPT_V3_SYSTEM` already calls out in `CRITICAL` bullets — e.g.
`rmi_data_create` called `RMI_ERROR_REALM` (a payload-carrying variant,
`RMI_ERROR_REALM(int)`) as a bare unit value; `rmi_pdev_create` confused two
near-identical struct names (`RmiPdevSpdm` vs. the real `RmmPdevSpdm`);
`rmi_realm_create` dropped `AddrInRange`'s leading `s: S` parameter — the
same class Iteration 6 already found and prompt-patched. Also, `old_s`/
`new_s` parameter ordering was inconsistent across commands in this run
(sometimes right after `result`, sometimes moved to the end of the
parameter list) in a way Haiku's output wasn't. Read together with
Iteration 6's finding: a more capable, more "confident" model that commits
to real symbol names and richer logic has *more* surface area to trip these
concrete, machine-diagnosable mistakes — CodeBLEU rewards the added realism
regardless of whether the details are right.

**Repair loop recovers it — to a new best: 100% pass rate.** Ran
`repair_loop_verus_claude.py --model claude-opus-4-8 --effort high --resume`
against the 51 failing commands on the GPU server. First attempt reported a
suspicious "51/51 resolved" summary paired with only 62.24% (61/98) on the
final Verus re-check — a contradiction that turned out to be a real bug in
how `--resume` was used: `results/ab_test/v3/alp14/*/repair_log.json`
still had 52 **stale** files left over from Iteration 6c's *Haiku* repair
run (timestamped hours before the Opus regeneration overwrote
`generated.*.rs` in the same directories), and `--resume`'s
`already_done()` check happily treated those as "already resolved" for the
*new* Opus failures without checking they applied to the current generated
code. Fixed by deleting the 52 stale `repair_log.json` files (predating the
Opus regeneration) and re-running; the 14 already-genuine Opus-repair
results from the first (partial) run were correctly reused via `--resume`,
and the remaining 37 commands were repaired for real:

| Run | CodeBLEU (all 98, recomputed locally) | Verus pass rate |
|---|---|---|
| Iteration 6c (Haiku, post-repair) | 0.5410 | 95/98 (96.94%) |
| Iteration 7a (Opus 4.8, fresh, no repair) | 0.5664 (Best@1) | 47/98 (47.96%) |
| **Iteration 7b (Opus 4.8, post-repair) — current best** | **0.6063** | **98/98 (100.00%)** |

**All 98 commands pass Verus verification** — the first 100% result in this
doc, beating Haiku's 95/98 and with meaningfully higher CodeBLEU too
(0.6063 vs 0.5410). Note the repair loop's own per-attempt `codebleu` field
in `repair_log.json` reads `0.0` throughout this run — that's a missing-
dependency artifact (`compute_codebleu()` silently returns `0.0` when the
`codebleu` PyPI package isn't importable, which is the case in the GPU
server's Python env used for the repair loop), not a real score; the 0.6063
figure above was recomputed locally (where `codebleu` is installed) directly
against `generated.formatted.rs` for all 98 commands post-repair, the same
methodology used for every other CodeBLEU number in this doc. Result
artifacts: `results/ab_test/v3/alp14/` (overwritten in place, Iteration 6's
Haiku artifacts no longer recoverable from this path — only from git history
if committed), `results/ab_test/v3/alp14_verus_check_summary.json`
(pre-repair, 47/98), `results/ab_test/v3/alp14_verus_check_summary_repaired.json`
(post-repair, 98/98).

**Takeaway:** this is now the second time in this doc (see Iteration 6) that
a change which *increases* CodeBLEU *decreases* raw Verus pass rate before
any repair — reinforcing that CodeBLEU should never be read as a correctness
signal on its own, model-to-model or version-to-version. But it's also the
second time a Verus-feedback repair loop fully absorbed that regression and
then some: Opus 4.8 + repair loop is a strict improvement over Haiku + repair
loop on both metrics (100% vs 96.94% pass rate, 0.6063 vs 0.5410 CodeBLEU),
at the cost of a more expensive model (Opus API pricing is 5x Haiku's) and a
real operational hazard now documented above (stale `--resume` caches across
model-swap regenerations) worth checking for on any future rerun.

### Iteration 7b, additional checks: dangling-output, footprint, Z3 inconsistency

100% Verus pass rate only means "every generated spec type-checks and is
internally provable" — it says nothing about whether the spec is
*complete* (declares every output the command actually has) or *logically
sound* (its own preconditions aren't self-contradictory). Ran the same
three rule-based checks used on our Qwen pipeline
([`OUR_CODE_RULE_CHECK.md`](OUR_CODE_RULE_CHECK.md)) against Iteration 7b's
final `results/ab_test/v3/alp14/` (all local checks; Z3 sweep run on the
GPU server since it needs the `verus` binary):

| Check | Command | Result |
|---|---|---|
| Dangling-output | `python3 training/scope_rule_check_ourcode.py --gen-dir results/ab_test/v3/alp14` | **3 flagged** (all confirmed real) |
| Footprint (naive) | (same run, footprint sub-check) | 1 flagged (`RMI_RTT_SET_S2AP`) |
| Footprint (semantic-normalized) | `python3 training/footprint_check_normalized.py --raw-file scope/alp14_raw.txt --gen-dir results/ab_test/v3/alp14` | **0 flagged** — the 1 naive flag resolves as a false positive (`s2ap_addr` matches after AST normalization) |
| Z3 `ensures false`, automated blind sweep | `inconsistency_analysis_model.py --results-root results/ab_test/v3/alp14 --specs-dir training-dataset/specs/alp14 --verus verus_src/verus-x86-linux/verus` (server) | **0/98 inconsistent, 98 consistent, 0 type-error** |
| Z3 `ensures false`, targeted witness (same method as `training/rmm_spec_bug_report.md`) | Hand-built `proof fn` with extra `requires` narrowing to a suspected overlap, checked against 1 candidate command (`RMI_RTT_SET_S2AP`) | **1/1 confirmed inconsistent** (see below) — also reproduces on the oracle spec, not Opus-specific |

**Dangling-output — 3 real bugs, worse than Haiku's 0.** Confirmed against
each command's oracle signature (not just "name missing from body" — these
commands' *signatures* omit declared oracle outputs entirely):
- `RMI_PSMMU_IRQ_NOTIFY`: oracle signature has `action, rd, vsmmu, msi_addr,
  msi_data` as outputs; Opus's generated signature has none of them — the
  body is nearly a stub (`!PsmmuAddrIsValid(...) ==> RMI_ERROR_INPUT`, no
  success-path constraints at all).
- `RMI_RTT_READ_ENTRY`: oracle has `desc: Bits64`; Opus substituted an
  invented `rtte: RmmRttEntry` field instead of matching the oracle's
  bit-encoded output.
- `RSI_MEM_SET_PERM_INDEX`: oracle has `new_cookie: Bits64`; Opus's
  signature omits it entirely.

These pass Verus because `verify_generated_verus.py`'s check only verifies
a spec is internally consistent with *its own* declared signature — it has
no way to know the signature itself is missing parameters relative to the
oracle. This is a real quality regression from Haiku (which had 0 flags on
this check) traded for the pass-rate/CodeBLEU wins above — see Iteration
7a's takeaway on Opus's higher "confidence" cutting both ways.

**Z3 automated blind sweep — 0/98, but this is not evidence of unusual
correctness.** The exact same check has returned 0 on *every* V3_PROMPT-based
general-model run tried in this project so far: Haiku (0/95, prior run),
GPT (0/65, `OUR_CODE_RULE_CHECK.md`), and now Opus (0/98). The one time this
*automated* sweep ever found a real bug in this project was on our
**fine-tuned Qwen** model (1/53, `psci_affinity_info` — see
`OUR_CODE_RULE_CHECK.md`'s Logical-inconsistency section), which used a
*different* generation style (learned from training data) than
`V3_PROMPT`'s explicit structural rule ("extract failure/success conditions
as implications, combine only with `&&`"). Qwen's bug was a bare
unconditional `&&` forcing two mutually exclusive `result` values
simultaneously — exactly the shape `V3_PROMPT`'s implication discipline
makes structurally hard to produce by accident. So the *automated* sweep's
positive rate tracks *prompt structure*, not model capability — it is close
to blind for any model following `V3_PROMPT` **when it only tests each
function against its own unconstrained parameters**, with no additional
hypotheses narrowing the search.

**Z3 targeted witness (same method as `training/rmm_spec_bug_report.md`) —
found a real bug on the first command checked.** Rather than the automated
sweep's generic "does `spec_fn(x)` ever hold for any `x`" probe, this method
(the one `rmm_spec_bug_report.md`'s Bugs 4–6 used) reads a command's PDF
failure-condition table and ordering statement, spots two conditions that
(a) map to different result codes and (b) have no stated priority ordering
between them, then hand-writes a `proof fn` with the two conditions as
*extra* `requires` hypotheses to force Z3 toward the specific suspected
overlap. Scanned Iteration 7b's 98 generated files for commands with 3+
distinct `RMI_ERROR_*`/`RSI_ERROR_*` targets and no apparent
mutual-exclusion guards between them; `RMI_RTT_SET_S2AP` was the first
candidate checked, and its PDF (§B4.3.45.2.1) reads: *"The RMI_RTT_SET_S2AP
command does not have any failure condition orderings"* — the identical
phrasing Bug 5 flagged for `RSI_ATTESTATION_TOKEN_CONTINUE`.

Witness: `rec_state` (`rec.state == REC_RUNNING` → `RMI_ERROR_REC`) and
`rd_align` (`!AddrIsGranuleAligned(rd)` → `RMI_ERROR_INPUT`) constrain
independent parameters (`rec_ptr` vs. `rd`) and have no stated ordering, so
nothing prevents both firing at once — which would require
`result == RMI_ERROR_REC` and `result == RMI_ERROR_INPUT` simultaneously.

```rust
proof fn bug_rmi_rtt_set_s2ap_dual_error(
    rd: Address, rec_ptr: Address, base: Address, top: Address,
    out_top: Address, rtt_tree: u64,
    result: Result<(), RmiStatusCode>, old_s: S, new_s: S,
)
    requires
        !AddrIsGranuleAligned(old_s, rd),
        RecAt(old_s, rec_ptr).state == REC_RUNNING,
        rmi_rtt_set_s2ap_spec(result, rd, rec_ptr, base, top, out_top, rtt_tree, old_s, new_s),
    ensures
        false,
{}
```

Verified against Opus's `generated.formatted.rs`: **`8 verified, 0 errors`**
— proved. Re-ran the identical two hypotheses against the **oracle**
`rmi_rtt_set_s2ap_spec` (different param order, same logic): also **`8
verified, 0 errors`** — the same contradiction, meaning this is a property
of the RMM spec's own failure-condition table (or the gold annotation's
literal transcription of it), not an Opus-specific defect; Opus's generated
code just faithfully reproduced the oracle's un-prioritized `&&` structure.
Control test — the same two hypotheses *without* the third (the call into
`rmi_rtt_set_s2ap_spec`) — does **not** prove `false` (`1 errors`),
confirming the witness state itself is reachable/satisfiable and the
contradiction genuinely comes from the spec, not from an unsatisfiable
proof setup.

This is a new, previously undocumented instance of the same bug class as
`rmm_spec_bug_report.md`'s Bug 5 — that report only covered 11 alp14
commands; `RMI_RTT_SET_S2AP` wasn't among them. It also reinforces the
takeaway above with a concrete example rather than just a theoretical
argument: the automated blind sweep's 0/98 genuinely undercounts real
inconsistencies — this bug was found on the *first* manually-selected
candidate tried, not after an exhaustive search, which suggests more may be
findable the same way across the other 97 commands (not yet done — see Not
yet done).

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
Unlike Claude's Iteration-6c/Haiku 3 remaining failures (all clean
`missing_symbol`) — or current Claude Opus 4.8 (Iteration 7b), which has
zero remaining failures — or our Qwen's mostly-`missing_symbol`/
`type_mismatch` spread, GPT's failures skew toward `type_mismatch` —
plausible-looking Rust that doesn't actually type-check against the
preamble's real signatures, plus a few outright `parse_error`s (malformed
function items).

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

| Metric | Claude Opus 4.8 (preamble + repair loop, Iteration 7b) | GPT `gpt-5.6-sol` (prompted only, preamble restored, no repair) | Our fine-tuned Qwen (+ repair loop) |
|---|---|---|---|
| CodeBLEU Best@1 | 0.6063 (all-98 post-repair; fresh-gen Best@1/3/5 was 0.5664/0.5937/0.6051) | 0.5720 / 0.6014 / 0.6285 (Best@1/3/5) | **0.8389** / — / — |
| Verus pass rate | **98/98 (100.00%)** | 65/98 (66.33%) | 47/98 (48.0%) |

(For reference, Iteration 6c's Claude 4.5 Haiku result — preamble + repair
loop, same prompt — was 0.5410 CodeBLEU / 95/98 (96.94%); Opus 4.8 beats it
on both metrics, see Iteration 7 above.)

Note the Claude and GPT numbers are no longer produced by identical
pipelines: Claude's went through the Iteration-6 preamble-restoration +
repair-loop cycle (Iteration 7 swapped the generator to Opus 4.8 on top of
that), GPT's is still single-shot best-of-5-CodeBLEU with no repair (see Not
yet done). Both now see the same preamble, but only Claude's failures got a
chance to be fixed against real Verus errors — keep that in mind reading the
gap below, and see Interpretation point 3.

GPT lands well behind Claude on both CodeBLEU (0.5720 vs Claude's 0.6063)
and Verus pass rate (66.33% vs 100.00%) — but is still clearly ahead of our
fine-tuned Qwen's 48.0% pass rate, despite Qwen's much higher CodeBLEU
(0.8389). So the CodeBLEU-vs-Verus disconnect isn't just a Claude-vs-Qwen
thing — it shows up again with a third, independently-prompted model, which
is further evidence CodeBLEU rank order and Verus-pass rank order genuinely
don't agree with each other across models, not just within one.

## Interpretation

This is a genuinely mixed, non-obvious result — worth stating plainly rather
than picking whichever metric favors "our" pipeline:

1. **On CodeBLEU, our fine-tuned model still wins**, but by a smaller margin
   now (0.84 Qwen vs 0.61 Claude Opus 4.8 vs 0.57 GPT). This makes sense:
   fine-tuning on the train split teaches the model to reproduce the oracle's
   exact style (naming, structure, type aliases), which is exactly what a
   text-similarity metric rewards — neither general model was ever trained on
   this oracle style, though Opus's repaired output closes some of that gap
   versus Haiku's 0.54.
2. **On actual correctness (Verus pass rate), all general models beat our
   fine-tuned pipeline**, and by very different margins: Claude Opus 4.8
   **100.00%** and GPT 66.33%, vs our Qwen's 48.0%. This is the more
   important metric, since it's a real compiler/verifier check, not a
   similarity proxy — and it directly confirms the finding already flagged
   in Iteration 7 of `RESULTS_V3.md`: **CodeBLEU is a poor proxy for
   Verus-verifiable correctness.** All three models produce
   plausible-looking Rust; only checking it against the actual verifier
   reveals how differently "plausible" and "correct" track each other per
   model. GPT sitting well below Claude on both metrics, while our
   fine-tuned Qwen has by far the *highest* CodeBLEU but the *lowest* pass
   rate of the three, is itself useful evidence that the disconnect is a
   property of the metric, not an artifact of any one model pair.
3. **The Claude/GPT gap (100.00% vs 66.33%) is on the same base prompt but
   *not* the same pipeline** — both use the identical `PROMPT_V3_SYSTEM`
   (with all Iteration 1–5/7 anti-hallucination rules), identical RAG setup,
   and (as of Iteration 6) an identical restored preamble block — but
   Claude's 98/98 also benefited from up to 10 rounds of Verus-feedback
   repair against its failures (51 of them, post-Opus-switch), while GPT's
   65/98 is still single-shot, no repair. So this specific gap is **not** a
   clean model-capability read — some of it is repair-loop access GPT hasn't
   had yet (see Not yet done). GPT's failures skew toward `type_mismatch`
   (19/33) rather than `missing_symbol` — the same class the repair loop's
   symbol-snippet grounding was effective against for Claude (both Haiku's
   and Opus's failures), suggesting GPT would likely close much of this gap
   if given the same repair loop rather than reflecting a hard capability
   ceiling.
4. **Model capability, not just fine-tuning, appears to be a real factor for
   our pipeline's gap too.** Claude (both Haiku and Opus 4.8) and GPT are all
   substantially larger/more capable general models than the 4B LoRA-tuned
   Qwen checkpoint, and all beat it on Verus pass rate despite zero
   fine-tuning. This is consistent with Iteration 7's finding (in
   `RESULTS_V3.md`) that our pipeline's remaining 51 failures have "no single
   dominant pattern" — i.e. may partly reflect a capability ceiling for a 4B
   model under this repair strategy, not just a prompt/data problem.
5. **The comparison is not fully apples-to-apples on repair**: Claude's
   number now includes up to 10 rounds of Verus-feedback repair (Iterations
   6c and 7b); GPT's `n_samples=5` CodeBLEU-based best-of-5 selection has no
   repair loop yet; our pipeline used single-sample generation plus up to 2
   rounds of repair. A fairer follow-up would give GPT the same repair-loop
   treatment Claude just got (see Not yet done) — until then, GPT vs
   Claude/Qwen specifically is a repair-loop-vs-no-repair-loop comparison,
   not a clean model-capability one.
6. **A stronger base model (Opus 4.8 vs Haiku) made the pre-repair result
   *worse*, not better** (47.96% vs 96.94% pass rate, despite higher
   CodeBLEU) — see Iteration 7's takeaway. This is a second, independent data
   point (after Iteration 6's preamble-restoration regression) that adding
   capability/context without a correctness-feedback loop can move CodeBLEU
   and Verus pass rate in opposite directions. The repair loop fully absorbed
   it here, but that shouldn't be assumed to always hold — it's the loop
   doing the correctness work, not the base model's one-shot output.

## Bottom line

For this specific goal — generating Verus specs that actually **pass
verification** — all general-purpose models beat our fine-tuned,
repair-augmented Qwen pipeline on real Verus pass rate (Claude Opus 4.8
**100.00%**, GPT 66.33%, vs our 48.0%), even though our pipeline wins on
CodeBLEU similarity (0.8389 vs 0.6063 / 0.5720). But Claude's number is no
longer a clean "general model, prompted only" result — it now reflects
prompt engineering *plus* a Verus-feedback repair loop (Iteration 6) *plus*
a stronger generator model (Opus 4.8, Iteration 7), the same repair-loop
lever our own pipeline uses. GPT hasn't had that lever applied yet, so the
66.33% vs 100.00% gap overstates the pure model-capability difference; it's
at least partly "had a repair loop" vs "didn't." This reinforces that
CodeBLEU should not be used alone to judge pipeline quality, and that a
pass-rate number isn't comparable across setups unless the repair budget is
also comparable — matching Iteration 6's own headline finding that "blind,
no-repair" and "sighted, with-repair" aren't an apples-to-apples comparison
even for the *same* model. Iteration 7 adds a further data point on the same
question: comparing the two most similar pre-repair setups (preamble
in-context, no repair loop yet) — Iteration 6b's Haiku at 40.82% (40/98) vs
Iteration 7a's Opus 4.8 at 47.96% (47/98) — Opus is somewhat better
pre-repair, but both are far below either model's post-repair number
(96.94% / 100.00%). The repair loop, not the choice of Haiku vs. Opus, is
doing most of the correctness work in both cases.

## Not yet done

- Run the same Verus-feedback self-repair loop against GPT's 33 remaining
  failures (script exists for Claude as
  [`repair_loop_verus_claude.py`](repair_loop_verus_claude.py), would need a
  GPT-model variant analogous to it), to make the Claude/GPT/Qwen head-to-head
  apples-to-apples on repair budget, not just on preamble access. Expected to
  close a meaningful chunk of GPT's gap given GPT's failures skew
  `type_mismatch`-heavy, the same class the repair loop's symbol-snippet
  grounding proved effective against for Claude.
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
- Extend the targeted Z3 witness method to the other 97 alp14 commands
  (only `RMI_RTT_SET_S2AP` checked so far, found 1/1) — scan each command's
  failure-condition table for "no ordering stated" + 2+ distinct result
  codes with independent-looking guards, same procedure used above and in
  `rmm_spec_bug_report.md`'s Bugs 4–6. Since the confirmed bug traces to the
  oracle's own structure, this should also be run against the full oracle
  spec set independent of any generator, and any confirmed finding added to
  `rmm_spec_bug_report.md` as a new numbered bug.
