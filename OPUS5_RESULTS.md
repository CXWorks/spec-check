# Claude Opus 5 on the alp14 Verus spec-generation benchmark

**Run date:** 2026-08-18 → 2026-08-20
**Generator:** `claude-opus-5`, `effort=high`, reached through the **`claude -p` CLI**
(Claude subscription auth), not the `anthropic` Python SDK.
**Benchmark:** alp14 (ARM CCA RMM 1.1-alp14), 98 commands, 5 samples/command.
**Prompt / RAG / repair loop:** unchanged from Iteration 7 (`PROMPT_V3_SYSTEM` +
`PROMPT_V3_TEMPLATE`, RAG top-3, `repair_loop_verus_claude.py --max-retries 10`).

This is Iteration 8 of
[`BASELINE2_GENERAL_MODEL_COMPARISON.md`](BASELINE2_GENERAL_MODEL_COMPARISON.md),
broken out as a standalone document. The only deliberate changes versus
Iteration 7 are the generator model and the transport.

---

## Headline

| Metric | **Opus 5 (Iter 8)** | Opus 4.8 (Iter 7) | Haiku 4.5 (Iter 6c) |
|---|---|---|---|
| CodeBLEU Best@1 / @3 / @5, fresh | **0.6445 / 0.6640 / 0.6723** | 0.5664 / 0.5937 / 0.6051 | — |
| Verus pass rate, fresh (no repair) | **62/98 (63.27%)** | 47/98 (47.96%) | 40/98 (40.82%) |
| Verus pass rate, post-repair | **98/98 (100.00%)** | 98/98 (100.00%) | 95/98 (96.94%) |
| CodeBLEU, post-repair | **0.6641 mean / 0.6963 median** | 0.6063 | 0.5410 |
| Repair effort to get there | **2.2 attempts mean, max 4** | up to 10 | up to 10 |

Two things are worth separating here.

**Opus 5 is the first generator change in this project where CodeBLEU and raw
Verus pass rate moved in the same direction.** Iterations 6 and 7 both raised
CodeBLEU while *lowering* the pre-repair pass rate — a more confident model
commits to real symbol names and gets their arity subtly wrong. Opus 5 breaks
that pattern: +15 points of fresh pass rate (47.96% → 63.27%) *and* higher
CodeBLEU. Only **1** of its 36 failures was `missing_symbol` (a fabricated
name); the rest are mechanical (`type_mismatch` 21, `verus_error` 10,
`parse_error` 4).

**But the post-repair ceiling did not move.** Both Opus 4.8 and Opus 5 land at
100.00%. What changed is the *cost* of getting there: mean 2.2 attempts
(attempt 1 is the already-failing generation, so most commands needed a single
corrective round), max 4, and zero commands needing more than 5 — where
Iteration 7b had commands running the full 10. Distribution: 30 commands
resolved at 2 attempts, 5 at 3, 1 at 4; **36/36 resolved, none unresolved**.

The honest reading: **the repair loop sets the final number, the generator sets
the price of reaching it.**

### The 100% was independently re-verified

Iteration 7's stale-cache incident surfaced as a disagreement between the
repair loop's self-report and a real re-check, so this run re-runs
`verify_generated_verus.py` from scratch over the final tree and writes a
separate summary. Both agree:

| Source | Result |
|---|---|
| Repair loop self-report (`..._repaired.json`) | 98/98 (100.00%) |
| Independent re-check (`..._recheck.json`) | 98/98 (100.00%) |

`already_done()` was additionally wrapped with a staleness guard
(`already_done_fresh()`), which ignores any `repair_log.json` older than the
`generated.raw.rs` it claims to describe — the exact failure mode Iteration 7
hit and had to fix by hand.

---

## Ground-truth bug reproduction

The important question is not "how does Opus 5 compare to Opus 4.8" but
"does it reproduce the bugs we have already machine-checked". Ground truth is
[`training/rmm_bugs.rs`](training/rmm_bugs.rs) /
[`training/rmm_spec_bug_report.md`](training/rmm_spec_bug_report.md).

Each was tested with `tools/targeted_witness.py`, which wraps the *generated*
spec in the real preamble and adds the ground-truth hypotheses as extra
`requires` alongside `ensures false`. Verus verifying that proof means the
hypotheses cannot hold together with the spec — a confirmed contradiction.

| Ground-truth finding | Section | GT status | Opus 5 result |
|---|---|---|---|
| **Bug 4** `RMI_PDEV_STOP` | §B4.3.20.2.1 | Confirmed spec bug | **Reproduced** (`8 verified, 0 errors`) |
| **Bug 5** `RSI_ATTESTATION_TOKEN_CONTINUE` | §B5.3.1.2.1 | Confirmed spec bug | **Reproduced** |
| **Bug 6** `RSI_VDEV_VALIDATE_MAPPING` | §B5.3.19.2.1 | **Annotation false positive** (ARM spec is correct) | **Reproduced the annotation defect** |
| `RMI_RTT_SET_S2AP` (found in Iter 7b) | §B4.3.45.2.1 | Confirmed, reproduces on oracle | **Reproduced** |

**3 of 3 in-scope ground-truth findings reproduce**, plus the Iteration-7b
find. The remaining two of the project's five confirmed bugs
(`SDEI_SHARED_RESET`, `SDEI_INTERRUPT_BIND`) and the DRTM one live in other
specs and are not part of the alp14 benchmark, so they are out of scope here.

**Bug 6 is the informative case.** Ground truth classifies it as an *annotation*
error, not a spec bug: ARM explicitly states `[da_en] < [vdev_id]`, so the real
spec is consistent — the gold Verus annotation is what dropped the priority
guard. Opus 5 made the same mistake, encoding
`VdevIdIsFree(...) ==> RSI_ERROR_INPUT` unconditionally instead of guarding it
with `feat_da == FEATURE_TRUE`. That is Opus 5 independently reproducing a
known annotation defect, **not** detecting a spec bug — and a reminder that a
"contradiction found" result must be read against the PDF before being called
a finding.

### Why the automated blind sweep found nothing

The blind sweep (`training/inconsistency_analysis_model.py`) reports
**0/98 inconsistent** (95 consistent, 3 trivial-`true` skips, 0 type-error) —
over the very same specs in which four targeted witnesses land immediately.

This is a property of the sweep, not evidence of soundness. It asks "does
`spec_fn(x)` hold for *any* `x`" with fully unconstrained parameters, which is
almost always satisfiable. Finding these contradictions requires added
hypotheses that force Z3 at a specific suspected overlap. Iteration 7b argued
this; this run demonstrates it directly. **Do not read `0/98` as "no bugs".**

(Cross-check: 95 consistent + 3 skipped + 0 type-error accounts for all 98,
which is consistent with the independent 98/98 Verus result. Running the same
sweep against the *oracle* specs yields 19 `type_error`s, because an oracle
spec is written against the full gold file rather than the standalone
`preamble.rs`; those same commands fail `verify_generated_verus.py --oracle`
identically.)

---

## Rule-based checks

| Check | Opus 5 | Opus 4.8 (Iter 7b) |
|---|---|---|
| Dangling-output | **1 flagged** (`RMI_PSMMU_IRQ_NOTIFY`) | 3 flagged |
| Footprint (naive) | 1 flagged | 1 flagged |
| Footprint (semantic-normalized) | **0 flagged** | 0 flagged |

Dangling-output improved from 3 to 1. `RMI_PSMMU_IRQ_NOTIFY` is the carry-over
— its generated signature omits the oracle's `action, rd, vsmmu, msi_addr,
msi_data` outputs entirely, leaving a near-stub body. Opus 4.8's other two
flags are fixed in Opus 5's output (`RMI_RTT_READ_ENTRY`'s invented
`rtte: RmmRttEntry` in place of the oracle's `desc: Bits64`, and
`RSI_MEM_SET_PERM_INDEX`'s missing `new_cookie`).

This matters for how to read the headline: **100% Verus pass does not mean the
specs are complete.** `verify_generated_verus.py` checks a spec against *its
own* declared signature and has no way to know the signature itself is missing
parameters relative to the oracle.

---

## Running it on a subscription plan

The run cost **$51.96** of metered value across **534 calls** and only
**2.7 h** of actual API time — but took **~3 days of wall clock**, because it
accumulated **95.7 h of quota waiting** across **161** suspend/probe/resume
cycles. Prompt caching kept per-call cost low (~$0.10/call, ~19k cached input
tokens once the byte-identical system prompt is warm).

**Two distinct limits exist**, and only one is the widely-known one:

- the rolling 5-hour session limit — `"You've hit your session limit · resets 12:30pm"`
- a **weekly** limit — `"You've hit your weekly limit · resets 8pm"` — which
  stalled this run for **~26 hours** with zero progress.

Budget for the weekly limit on any subscription-based rerun; it was the single
largest item in the schedule, far outweighing model latency.

### What made it survivable

`prompt_engineering/claude_cli_model.py` (`ClaudeCLIModel`) is a drop-in for
`ClaudeModel` that adds:

- **Quota suspend → probe → resume.** Parses the reset time out of the limit
  message (including its IANA timezone), sleeps until then, probes cheaply, and
  retries the same request. Never gives up; suspension is normal operation.
- **A call-granular disk cache**, keyed by `sha256(model+effort+system+user)`,
  storing an *ordered list* per key so the *k*-th repeat of an identical prompt
  (`n_samples=5` sends the same prompt 5×) consumes the *k*-th cached response.
  The run was restarted five times mid-flight without losing a completed call.
- **A per-call JSONL ledger** (`cli_calls.jsonl`) with duration, cost, tokens,
  cache hits and quota waits — the source for every figure in this section.

`tools/prewarm_cache.py` adds a second worker that walks the command list in
reverse through the *same* `evaluate_prompt_variant` code path, so its prompts
— and therefore its cache keys — are byte-identical to the main run's (verified
against 6 already-completed commands before launch). It roughly doubled
throughput during a degraded-API stretch.

### Defects found in the harness during the run

All five surfaced from production data, not review:

1. **`529 Overloaded` classified as quota exhaustion.** 429 and 529 were
   lumped together; 529 means "server busy, retry shortly", so this scheduled
   10-minute sleeps for something that clears in seconds.
2. **The real limit-message format was unparseable.** The live text is
   `resets 7am (America/New_York)` — no "at" — so every window fell back to
   blind backoff instead of sleeping exactly until reset.
3. **The 2-hour sleep-chunk cap discarded a known reset time**, so it could
   probe at 12:15 for a 12:30 reset, miss, and sleep to 13:15.
4. **A reset time already in the past rolled forward 24 hours** instead of
   being treated as "window should be open now".
5. **An `ERROR` sample in slot 0 silently zeroed a command's `Best@1`**
   (Best@1 reads the *first* sample). Sustained overload now retries
   indefinitely rather than emitting an ERROR sample.

### Methodology delta worth recording

The CLI exposes no sampling parameters, so the repair loop's temperature
schedule cannot apply. This is **not** a regression versus Iteration 7:
`ClaudeModel.supports_sampling_params` is `name.startswith("claude-haiku")`, so
the schedule was already inert on Opus 4.8. In CLI mode the loop varies the
prompt text per attempt instead ("attempt N of M … take a materially different
approach"), and the `seen_fmt` cycle-break still applies.

---

## Reproducing

```bash
export VERUS_BIN=/path/to/verus
export VERUSFMT_BIN=/path/to/verusfmt
./run_opus5_eval.sh          # 7 resumable stages; re-invoke after any interruption
```

Stage 1 alone:

```bash
python3 prompt_engineering/prompt_engineering_v3.py \
    --split test --limit 98 --n-samples 5 \
    --rag-index rag/index.json --rag-top-k 3 --save-results --resume \
    --backend cli --model claude-opus-5 --effort high --variant-key v3_opus5
```

Stages: 1 generation → 2 pre-repair snapshot → 3 Verus check → 4 repair loop →
5 independent re-check + CodeBLEU → 6 dangling-output + footprint →
7 Z3 blind sweep. The pre-repair snapshot exists so the fresh-generation
numbers stay recomputable; Iteration 7 lost its equivalent by repairing in
place.

### Artifacts

| Path | Contents |
|---|---|
| `results/ab_test/v3_opus5/alp14/` | Final (post-repair) per-command specs — gitignored, bulky |
| `results/ab_test/v3_opus5/alp14_prerepair/` | Pre-repair snapshot — gitignored |
| `alp14_verus_check_summary.json` | Pre-repair, 62/98 |
| `alp14_verus_check_summary_repaired.json` | Repair loop self-report, 98/98 |
| `alp14_verus_check_summary_recheck.json` | Independent re-check, 98/98 |
| `alp14_inconsistency_sweep.json` | Z3 blind sweep |
| `opus5_scores.json` | All CodeBLEU / pass-rate / repair / cost figures |
| `cli_calls.jsonl` | Per-call ledger |

New code: `prompt_engineering/claude_cli_model.py`, `run_opus5_eval.sh`,
`tools/{score_opus5,targeted_witness,prewarm_cache}.py`,
`training/inconsistency_analysis_model.py` (referenced by Iteration 7b but
missing from the checkout; reconstructed here).

---

## Limitations

- **The targeted-witness scan is not exhaustive.** 33 commands have 3+ distinct
  error targets with no mutual-exclusion guard; 4 were tested (the ground-truth
  set). The other 29 are untested — the "4 confirmed" figure is a lower bound,
  not a census.
- **100% pass ≠ correct specs.** It means every spec type-checks and is
  internally provable against its own signature. One dangling-output bug
  remains, and `RMI_RTT_SET_S2AP` is Z3-provably self-contradictory in both the
  generated spec *and* the ARM oracle.
- **CodeBLEU is not a correctness signal.** This doc's own history has two
  cases where it rose while the pass rate fell.
- **Not a controlled model comparison.** Opus 5 changed transport *and* model
  versus Iteration 7, so the two are not perfectly isolated — though the prompt,
  RAG setup, repair loop, and benchmark are identical.
