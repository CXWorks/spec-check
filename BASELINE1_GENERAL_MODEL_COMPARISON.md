# Baseline 1 with General SOTA Models: GPT `gpt-5.6-sol` and Claude Opus 5

**Goal**: run the Baseline 1 experiment from
[`BASELINE1_SCOPE_REPRODUCTION.md`](BASELINE1_SCOPE_REPRODUCTION.md) — generate Verus
specs for eac5/rel0 and check whether SCOPE's dangling-output rule, applied to *our
generated code*, rediscovers the bugs SCOPE originally found — but with two
general-purpose SOTA models in place of our fine-tuned Qwen. Both were driven through
their subscription CLIs (`codex exec`, `claude -p`), not API keys.

## Headline result

| Generator | SCOPE TPs flagged (of 8) | Genuine spec-gap rediscoveries | Extra flags (FP) | CodeBLEU Best@1 (eac5 / rel0) |
|---|---|---|---|---|
| Qwen `item_split_v4_best` (published, not recomputed here) | 7/8 | **5** | 1 (`RMI_RTT_FOLD`) | 0.8139 / 0.8056 |
| GPT `gpt-5.6-sol` (high) | **7/8** eac5, **7/8** rel0 | **7** | 1 (`RSI_FEATURES`) | 0.6634 / 0.6403 |
| Claude Opus 5 (high) | **7/8** eac5, **7/8** rel0 | **7** | 1 (`RSI_FEATURES`) | 0.6348 / 0.6299 |

Field-level recall for both SOTA models: **16/17** on eac5, **16/18** on rel0.

Read the two middle columns together — they measure different things:

- **"SCOPE TPs flagged"** is the objective join: did the check flag a command SCOPE's own
  labelling patch marks `XXX: TP`? All three generators score 7/8; all three miss the
  same one, `RSI_IPA_STATE_GET` for Qwen and `RMI_RTT_READ_ENTRY` for the SOTA models.
- **"Genuine spec-gap rediscoveries"** is the source-verified subset: the spec's own
  structured conditions table never defines the output, so nobody could translate it.
  This is the column `BASELINE1_SCOPE_REPRODUCTION.md` reports as "5 of 8". For the SOTA
  models all 7 flags survive that check, verified two ways — the conditions table never
  names the output, and the generated code establishes it under no alternative accessor
  name either (both emit a bare `true` body for `RMI_VERSION`/`RSI_VERSION`, so there is
  no `rsi_version_lower(...)`-style definition hiding from the literal-name check).

The Qwen row's 5 is lower than its 7 flags for reasons specific to that run, both
documented in the original report: `RSI_VERSION` was a false positive of the checker
(the model *did* define the value, via `rsi_version_lower(...)`), and `RMI_VERSION` was
discounted as a generation inconsistency rather than a spec-gap rediscovery.

## The one miss is the interesting finding

Both SOTA models fail on exactly the same true positive, on both spec versions, for the
same reason — and it is not carelessness.

`RMI_RTT_READ_ENTRY`'s Success-conditions table contains **zero** occurrences of
`walk_level` (that absence *is* SCOPE's bug), and the gold oracle correspondingly leaves
it undefined. Both models invented a definition anyway:

```rust
// GPT gpt-5.6-sol
walk_level as int == RttWalk(new_s, rd, ipa).level
// Claude Opus 5
walk_level as int == RttWalk(old_s, rd, ipa).level
```

Four independent generations produced four different spellings of a definition that
exists nowhere in the spec:

| Source | `walk_level` in the spec fn body | Flagged? |
|---|---|---|
| Gold oracle, eac5 / rel0 | *not established* | yes |
| GPT, eac5 | `walk_level as int == RttWalk(new_s, rd, ipa).level` | no |
| GPT, rel0 | `walk_level == ToBits64(RttWalk(old_s, rd, ipa).level)` | no |
| Claude, eac5 | `walk_level as int == RttWalk(old_s, rd, ipa).level` | no |
| Claude, rel0 | `walk_level == RttWalk(old_s, rd, ipa).level` | no |

`new_s` vs `old_s`, an `as int` cast vs a `ToBits64(...)` wrapper — no copying between
models, each independently reached for the same plausible semantics and wrote it down.

The same thing happens on rel0's `RSI_IPA_STATE_GET`/`out_top`, whose Success conditions
section reads in full *"The RSI_IPA_STATE_GET command does not have any success
conditions"* and whose gold annotation constrains nothing. GPT emitted
`out_top > base`, `out_top <= top`; Claude emitted
`(out_top as int) > (base as int) && (out_top as int) <= (top as int)` — the same
invention, differing only in casts. This is a field-level miss, which is why rel0 field
recall is 16/18 while command-level recall stays 7/8: `ripas` is still flagged for that
command, so the command counts as detected.

**A stronger generator hides spec bugs by confabulating plausible definitions for outputs
the specification never defines.** The weaker fine-tuned model leaves the gap visible,
and that visibility is what makes the gap detectable. Two independent SOTA models
reproducing the identical failure mode on the identical commands makes this a property of
capable models on this task, not a sampling artifact.

This cuts against the intuition that a better generator yields a better bug detector: on
CodeBLEU the SOTA models score ~0.20 *below* the fine-tuned Qwen, yet flag as many
SCOPE TPs — and the one they miss, they miss precisely *because* they are more capable.

## The false positive is caused by a bug in our own prompt

Both models flag `RSI_FEATURES`/`value` on both versions — the opposite failure to the
miss above. Here the spec **does** define the output (`value == Zeros()` appears in the
conditions table) and both models still failed to encode it:

| Source | eac5 and rel0 body |
|---|---|
| Gold oracle | `(result == RSI_SUCCESS ==> value == 0)` |
| GPT | `true` |
| Claude | `true` |

So this is a **generation defect, not a spec bug** — the distinction the dangling-output
check cannot make on its own, and the reason every flag outside the ground truth needs
source verification before it is reported as a finding.

Cause — `PROMPT_V3_SYSTEM` (`prompt_engineering/prompt_engineering_v3.py:65`) asserts:

> Examples: PSCI_FEATURES (fully unconstrained, oracle returns `true`), **RSI_FEATURES**,
> version queries → all should return `true`.

That is wrong for every version in the dataset:

| Version | `rsi_features_spec` oracle |
|---|---|
| eac5, rel0 | `result == RSI_SUCCESS ==> value == 0` |
| alp13, alp14 | `result == RSI_SUCCESS ==> value == RsiFeatureRegisterEncode(...)` |

Because `training/build_dataset.py` imports this same prompt as the training system
prompt, the error is baked into the fine-tuned Qwen's weights too — the same class of
defect as Iteration 7's Part B.2 finding in `prompt_engineering/RESULTS_V3.md`. **It was
deliberately left unfixed for this run** so the two SOTA models and the published Qwen
numbers stay comparable; fixing it is a follow-up that should trigger a retrain.

## Setup: what is actually comparable

| | GPT `gpt-5.6-sol` (high) | Claude Opus 5 (high) | Qwen `item_split_v4_best` |
|---|---|---|---|
| Access | `codex exec -m gpt-5.6-sol -c model_reasoning_effort=high` | `claude -p --model claude-opus-5 --effort high --tools ""` | local GPU inference |
| Adaptation | prompt only, no fine-tuning | prompt only, no fine-tuning | LoRA fine-tuned on the train split |
| Prompt | `PROMPT_V3_SYSTEM` verbatim + preamble + RAG top-3 | identical | same system prompt, no preamble at inference |
| Samples | n=1 (round 1) | n=1 (round 1) | n=1 |
| Commands | 41 eac5 + 41 rel0 | 41 eac5 + 41 rel0 | 41 + 41 |
| Wall time | 48 min (avg 35.2 s/call) | 19 min (avg 14.1 s/call) | — |
| Artifacts | `results/baseline1_general/gpt56sol/` | `results/baseline1_general/claude_opus5/` | server-side, not in this repo |

The preamble (200-line tail of `specs/{version}/preamble.rs`) is **restored** in the
template for both SOTA models — Iteration 6 removed it because the fine-tuned Qwen had it
baked into training, but a general model has never seen this DSL and would otherwise be
guessing at struct/enum/helper names. This matches the call Baseline 2 made for GPT.

One unavoidable asymmetry: `claude -p --tools ""` is a clean single-turn completion with
our system prompt installed as *the* system prompt, while `codex exec` wraps the request
in its own agent harness whose system prompt cannot be replaced, so ours is prepended to
the user message instead. The user-visible instructions are identical.

## Ground truth and checker validation

Neither the bug list nor the checker was taken on faith.

1. **Ground truth re-derived from SCOPE itself**, not copied from the paper:
   `./scope --target {eac5,rel0} --input-type txt --mode rule`, then SCOPE's own
   labelling patch `patch/{target}_rule.patch`, parsed by
   `training/parse_scope_rule_output.py` into
   `training/scope_ground_truth_eac5_rel0.json`. The result matches the 8-row table in
   `BASELINE1_SCOPE_REPRODUCTION.md` exactly, including the rel0-only `out_top` field and
   the `desc` field that SCOPE's own patch labels `FP`.
   The input `eac5.txt`/`rel0.txt` were verified byte-identical to fresh
   `pdftotext -layout` output from the ARM PDFs.

2. **Checker control run on the gold oracle specs**: `8/8` on eac5 and `8/8` on rel0 with
   **zero** extra flags. Any deviation in a model's run is therefore attributable to the
   model's code, not to the check.

3. **A real bug in the checker was found and fixed during this work.**
   `extract_our_clauses` dropped only the *first line* to skip the function signature.
   verusfmt wraps long signatures across several lines, so parameter names leaked into
   the text searched for "does the body mention this output" — making a function whose
   entire body is `true` look like it defines every declared output. GPT's first eac5 pass
   scored a nonsense 0 flags because of it. Fixed in
   `training/scope_rule_check_ourcode.py` by splitting at the brace that opens the body
   (`split_signature_body`). Regression-checked both ways: the gold control is unchanged
   (8/8 + 8/8, zero extras), and the committed Qwen alp14 run is **byte-identical** before
   and after (10 flagged commands, same set) because all 98 of its signatures happened to
   be single-line, which is why the bug never surfaced previously.

   Note: that alp14 re-check yields 10 flagged commands where
   `OUR_CODE_RULE_CHECK.md` reports 11. The difference is `RMI_FEATURES`, whose committed
   Qwen output writes `new_s.value == RmiFeatureRegisterEncode(...)` — the literal name
   `value` is present, so it is not dangling by this check.

## Caveats

- **The Qwen row is quoted from `BASELINE1_SCOPE_REPRODUCTION.md`, not recomputed.** Its
  generated code (`results/baseline1_eac5_rel0/v4_qwen/`) lives on the GPU server and is
  not in this repository, so its flags could not be re-derived with the fixed checker. Its
  CodeBLEU was also computed on a different machine, possibly with a different verusfmt;
  numbers here used verusfmt 0.7.2.
- **eac5 and rel0 are in the Qwen model's *training* split.** Both SOTA models saw them
  cold. The comparison structurally favours Qwen, and its higher CodeBLEU should be read
  in that light.
- **No Verus verification in this round.** There is no `verus` binary on this machine and
  building one is a multi-hour task. Baseline 1 never required it — the dangling-output
  check operates on the generated text — but this means these results say nothing about
  whether the generated specs compile.
- **n=1.** Rounds 2 and 3 (best-of-3) have not been run.
- `reason`-mode reproduction (`RMI_DATA_DESTROY`, `RMI_RTT_DESTROY`,
  `RMI_RTT_INIT_RIPAS`) remains not started, as in the Qwen report.

## Reproducing

`work/` is a local scratch directory and is gitignored; step 0 regenerates it from the
submodule and the ARM PDFs. The resulting `*_raw.txt` / `*_rule.txt` are also committed
under `benchmark/rule_check_8bugs/scope_tables/`, so steps 2+ can be run without it.

```bash
# 0a. stage a SCOPE run dir (needs the DEN0137 eac5/rel0 PDFs and pdftotext)
git submodule update --init                       # scope/ is a submodule
mkdir -p work/scope_run && cp scope/scope work/scope_run/ && cp -r scope/patch work/scope_run/
pdftotext -layout <DEN0137_1.0-eac5_rmm-arch_external.pdf> work/scope_run/eac5.txt   # same for rel0
cd work/scope_run
for v in eac5 rel0; do
  ./scope --target $v --input-type txt --mode raw  > ${v}_raw.txt
  ./scope --target $v --input-type txt --mode rule > ${v}_rule.txt
  patch -p0 < patch/${v}_rule.patch                # applies SCOPE's own TP/FP labels
done; cd ../..

# 0b. ground truth + checker control (control must print 8 flagged, 0 extra, per version)
python3 training/parse_scope_rule_output.py \
    --rule-file work/scope_run/eac5_rule.txt --version eac5 \
    --rule-file work/scope_run/rel0_rule.txt --version rel0 \
    --out training/scope_ground_truth_eac5_rel0.json
python3 training/scope_rule_check_ourcode.py \
    --raw-file work/scope_run/eac5_raw.txt \
    --gen-dir training-dataset/specs/eac5 --gen-pattern '{cmd}_spec.rs'

# 1. generate (idempotent; re-run the same line to resume after a usage limit)
python3 run_baseline1_general.py --model codex  --round 1 --versions eac5 rel0
python3 run_baseline1_general.py --model claude --round 1 --versions eac5 rel0

# 2. check + compare
python3 training/scope_rule_check_ourcode.py \
    --raw-file work/scope_run/eac5_raw.txt \
    --gen-dir results/baseline1_general/gpt56sol/eac5 \
    --json-out results/baseline1_general/gpt56sol/eac5_dangling.json
python3 training/baseline1_compare.py \
    --ground-truth training/scope_ground_truth_eac5_rel0.json \
    --findings eac5=results/baseline1_general/gpt56sol/eac5_dangling.json \
    --findings rel0=results/baseline1_general/gpt56sol/rel0_dangling.json \
    --label "GPT gpt-5.6-sol (high)" \
    --md-out results/baseline1_general/gpt56sol/rule_check.md
```

Per-model detail tables: `results/baseline1_general/{gpt56sol,claude_opus5}/rule_check.md`.

**Packaged benchmark**: [`benchmark/rule_check_8bugs/`](benchmark/rule_check_8bugs/) is a
self-contained version of this evaluation — ground truth, both models' generations, the
gold-oracle control, and a `score.py` that reproduces every number above from that
directory alone (`python3 score.py`). Verified: the control scores 16/16 with zero false
alarms, and both models re-score to 14/16.
