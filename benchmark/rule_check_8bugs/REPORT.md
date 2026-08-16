# SCOPE Rule-Mode Bug Benchmark — Results

Evaluation of two general SOTA models against SCOPE's 8 labelled rule-mode
dangling-output bugs, on `eac5` and `rel0` (16 positive items total).

Every number below is reproduced by `python3 score.py` from the data in this
directory alone.

## Results

| Generator | Recall (of 16) | eac5 | rel0 | Field recall | False alarms | CodeBLEU@1 (eac5 / rel0) |
|---|---|---|---|---|---|---|
| **Gold oracle** (control) | **16/16** | 8/8 | 8/8 | 17/17, 18/18 | **0** | — |
| GPT `gpt-5.6-sol` (high) | **14/16** | 7/8 | 7/8 | 16/17, 16/18 | 2 | 0.6634 / 0.6403 |
| Claude Opus 5 (high) | **14/16** | 7/8 | 7/8 | 16/17, 16/18 | 2 | 0.6348 / 0.6299 |
| Qwen `item_split_v4_best` † | 7/16 flagged, **5 source-verified** | — | — | — | 1 | 0.8139 / 0.8056 |

† Quoted from `BASELINE1_SCOPE_REPRODUCTION.md`; its generated code lives on the GPU
server and is not in this repository, so it could not be re-scored with this scorer. Its
"5" is a source-verified subset of 7 flagged — see *Two ways to count* below.

Cost: 82 calls per model, all succeeded. GPT 48 min (avg 35.2 s/call), Claude 19 min
(avg 14.1 s/call).

## The two models are indistinguishable on this benchmark

Identical recall, identical misses, identical false alarms, on both versions. The only
separation is CodeBLEU (GPT ~0.03 higher) and wall time (Claude 2.5× faster). If the
goal is telling these two apart, this benchmark does not do it — 16 items with 14
detected by both leaves no resolving power.

## Both miss the same item, and the reason is the interesting finding

`RMI_RTT_READ_ENTRY` / `walk_level` fails for both models on both versions — not through
carelessness. The spec's Success-conditions table contains **zero** occurrences of
`walk_level`; that absence *is* SCOPE's bug, and the gold oracle correspondingly leaves
the output undefined. Both models invented a definition anyway:

```rust
// GPT gpt-5.6-sol
walk_level as int == RttWalk(new_s, rd, ipa).level
// Claude Opus 5
walk_level as int == RttWalk(old_s, rd, ipa).level
```

Same on rel0's `RSI_IPA_STATE_GET` / `out_top`, where the oracle constrains nothing and
both models emitted bounds (`out_top > base && out_top <= top`) — which is why field
recall is 16/18 on rel0 but command-level recall is still 7/8.

**A more capable generator hides spec bugs by confabulating plausible definitions for
outputs the specification never defines.** The weaker fine-tuned model leaves the gap
visible, and that visibility is what makes the gap detectable. Two independent SOTA
models reproducing the identical failure mode on the identical items makes this a
property of capable models on this task, not sampling noise.

Note the inversion: both SOTA models score ~0.17 *below* the fine-tuned Qwen on CodeBLEU
yet detect more items — and the one they miss, they miss *because* they are more capable.

## The false alarm is caused by our own prompt

Both models flag `RSI_FEATURES` / `value` on both versions. The spec's table states
`value == Zeros()` and the oracle encodes it (`result == RSI_SUCCESS ==> value == 0`),
but both models emitted `true`. Cause — `prompt_engineering/prompt_engineering_v3.py:65`
asserts:

> Examples: PSCI_FEATURES (fully unconstrained, oracle returns `true`), **RSI_FEATURES**,
> version queries → all should return `true`.

That is wrong for every version in the dataset (eac5/rel0: `... ==> value == 0`;
alp13/alp14: `... ==> value == RsiFeatureRegisterEncode(...)`). Because
`training/build_dataset.py` imports the same prompt as its training system prompt, the
error is baked into the fine-tuned Qwen's weights too. It was left unfixed for this run so
the two models and the published Qwen numbers stay comparable.

## Two ways to count, and why they differ

- **Recall against SCOPE's labels** (what `score.py` reports): was a command SCOPE marked
  `XXX: TP` flagged? All three generators reach 7/8 per version.
- **Source-verified rediscoveries** (what `BASELINE1_SCOPE_REPRODUCTION.md` reports as
  "5 of 8"): of those flags, how many are genuine spec gaps where the structured
  conditions table defines the output for nobody? For both SOTA models all 7 survive,
  verified two ways — the table never names the output, and the generated code establishes
  it under no alternative accessor name either (both emit a bare `true` body for
  `RMI_VERSION`/`RSI_VERSION`, so there is no `rsi_version_lower(...)`-style definition
  hiding from the literal-name check). The Qwen run's 5 is lower than its 7 flags because
  `RSI_VERSION` was a checker false positive there (that model *did* define the value) and
  `RMI_VERSION` was discounted as a generation inconsistency.

## Case detail

Every clause below is quoted verbatim from `predictions/`, with whitespace normalised;
each is reproducible by opening the named file.

### Miss 1 — `RMI_RTT_READ_ENTRY` / `walk_level` (both models, both versions)

The spec's Failure+Success conditions tables contain **0 occurrences** of `walk_level`,
in eac5 and in rel0 alike — that absence *is* the bug SCOPE reports. The hand-written
gold annotation correspondingly never establishes the output.

| Source | `walk_level` in the spec fn body | Flagged? |
|---|---|---|
| Gold oracle, eac5 | *not established* | ✅ yes |
| Gold oracle, rel0 | *not established* | ✅ yes |
| GPT, eac5 | `walk_level as int == RttWalk(new_s, rd, ipa).level` | ❌ no |
| GPT, rel0 | `walk_level == ToBits64(RttWalk(old_s, rd, ipa).level)` | ❌ no |
| Claude, eac5 | `walk_level as int == RttWalk(old_s, rd, ipa).level` | ❌ no |
| Claude, rel0 | `walk_level == RttWalk(old_s, rd, ipa).level` | ❌ no |

Four independent generations, four different spellings — `new_s` vs `old_s`, an
`as int` cast vs a `ToBits64(...)` wrapper — of a definition that exists nowhere in the
specification. The models are not copying each other or a training artifact; each
independently reached for the same plausible-looking semantics ("the walk level is the
level the RTT walk reached") and wrote it down. That invention is what makes the output
non-dangling and hides the gap from the check.

### Miss 2 — `RSI_IPA_STATE_GET` / `out_top` (both models, rel0 only)

This is a *field-level* miss, which is why rel0 field recall is 16/18 while command-level
recall stays 7/8: `ripas` is still correctly flagged for the same command, so the command
counts as detected. rel0's ground truth lists both `out_top` and `ripas`; eac5's lists
only `ripas`, hence 16/17 there.

`RSI_IPA_STATE_GET`'s Success conditions section reads, in full:

> **B5.3.5.3 Success conditions** — The RSI_IPA_STATE_GET command does not have any
> success conditions.

`out_top` appears **0 times** in the conditions tables, and the gold oracle leaves it
unconstrained. Both models invented the same bound:

| Source | `out_top` in the spec fn body |
|---|---|
| Gold oracle | *not established* |
| GPT | `out_top > base`, `out_top <= top` |
| Claude | `(out_top as int) > (base as int) && (out_top as int) <= (top as int)` |

Semantically identical, differing only in casts and formatting.

### False positive — `RSI_FEATURES` / `value` (both models, both versions)

The opposite failure. Here the spec **does** define the output — `value == Zeros()`
appears in the conditions table, and the gold oracle encodes it — but both models emitted
a bare `true`:

| Source | eac5 and rel0 body |
|---|---|
| Gold oracle | `(result == RSI_SUCCESS ==> value == 0)` |
| GPT | `true` |
| Claude | `true` |

Cause is our own prompt, not the models: `PROMPT_V3_SYSTEM` names `RSI_FEATURES` in its
"fully unconstrained specs" rule and instructs that it return `true`. Both models obeyed.
This is a **generation defect, not a spec bug** — the distinction the check cannot make on
its own, which is why flagged items outside the ground truth need source verification
rather than being reported as findings.

## Validity controls

1. **Ground truth re-derived from SCOPE itself**, not transcribed from the paper:
   `scope --mode rule` plus SCOPE's own labelling patch `patch/{version}_rule.patch`,
   parsed by `training/parse_scope_rule_output.py`. Matches the 8-row table in
   `BASELINE1_SCOPE_REPRODUCTION.md` exactly, including the rel0-only `out_top` field and
   the `desc` field SCOPE's patch labels FP.
2. **Input provenance**: the `eac5.txt`/`rel0.txt` fed to SCOPE are byte-identical to fresh
   `pdftotext -layout` output from the ARM PDFs.
3. **Gold-oracle control**: scoring the hand-written reference annotations gives 16/16 with
   zero false alarms. Any deviation in a model's run is therefore attributable to the
   model's code, not to the check. Re-run it with `python3 score.py --model _gold_oracle`.
4. **A real bug in the checker was found and fixed while building this.**
   The original `extract_our_clauses` dropped only the *first line* to skip the function
   signature. verusfmt wraps long signatures across several lines, so parameter names
   leaked into the text searched for "does the body mention this output" — making a
   function whose entire body is `true` look like it defined every declared output. GPT's
   first eac5 pass scored a meaningless 0 flags because of it. Fixed by splitting at the
   brace that opens the body (`split_signature_body`, carried into `score.py`).
   Regression-checked both ways: the gold control is unchanged, and the committed Qwen
   alp14 run is byte-identical before and after (10 flagged commands, same set) because
   all 98 of its signatures happened to be single-line — which is why the bug never
   surfaced previously.

## Limitations

- **16 items is too few to separate models.** Both SOTA models tie at 14.
- **Heuristic, not machine-checked.** These are SCOPE's rule-mode findings; the
  dangling-output check is a name-occurrence test, not a Verus proof. The Verus-verified
  benchmark is a separate artifact (`benchmark/` alp14 work).
- **The Qwen row is quoted, not re-scored** — its generations are not in this repository,
  and its CodeBLEU came from a different machine with a possibly different verusfmt.
- **eac5/rel0 are in the Qwen model's training split**; both SOTA models saw them cold.
  The comparison structurally favours Qwen, which is worth remembering when reading its
  higher CodeBLEU.
- **n=1.** One sample per command per model; no best-of-k.
