# Unattended run, 2026-08-18

What changed, what it means, and what is still open. Every number here was
verified against `kubectl` / the HF API / a local file, never against the
background monitor, which produced unusable output five times (fabricated error
lines, a false Complete, Pending reported as Running).

---

## The headline: a claim was withdrawn

`BASELINE1_GENERAL_MODEL_COMPARISON.md` reads the `walk_level` result as
**capable models hide spec gaps by confabulating**. Two experiments say it is a
prompting artifact instead.

| generator | eac5 | rel0 |
|---|---|---|
| gold | blank | blank |
| `sft3-0` (4B, fine-tuned) | blank | blank |
| `sft3-2` (9B, fine-tuned) | blank | blank |
| Claude Opus 5, base prompt | **invents** | **invents** |
| Claude Opus 5, **+ one paragraph** | blank | blank |
| GPT `gpt-5.6-sol`, base prompt | **invents** | **invents** |

Capability varies down the rows and changes nothing — the 9B is stronger than the
4B and behaves identically. The prompt varies within one model and flips it on
both versions. The added paragraph names no command and no field.

Full 82-command scores: **Claude goes 14/16 to 16/16**, matching gold and matching
our 9B. So a properly-prompted general model has no detection deficit here.

The honest statement is that this task has a convention — *when the document says
nothing, say nothing* — that the fine-tuned models absorbed from ~250 examples and
the general models were never told.

---

## `sft3-*`: decontaminated and equivalent

Every `sft2-*` run was trained on the commands both benchmarks score.
`dataset_bench` holds all 17 out at every version: 249 command examples instead of
293, held-out set 49 with the old 40 as a strict subset.

| | shared 40 | 49 | McNemar |
|---|---|---|---|
| `sft2-0` 4B | 14/40 | — | — |
| `sft3-0` 4B | 16/40 | 18/49 (36.7%) | p = 0.625 |
| `sft2-2` 9B | 16/40 | — | — |
| `sft3-2` 9B | 20/40 | 22/49 (44.9%) | p = 0.344 |

The 15% cut costs nothing. The worry that dropping RTT commands would gut that
family was unfounded — `sft2-0`, trained on all of them, already scored 0/9 on RTT.

Correctness (`semantic_equiv` vs gold):

| | compiles | correct | weaker |
|---|---|---|---|
| `sft3-0` 4B | 18 | 10 (20.4%) | 1 |
| `sft3-2` 9B | 22 | 11 (22.4%) | **0** |

The 9B's zero `weaker` is its only defensible advantage: that is the failure a
compile check structurally cannot see. **It does not survive `--with-preamble`**
— that condition takes the 9B to 1 `weaker` and the 4B to 2, while raising both
models' compile rate. See the preamble section below.

### Seed spread differs by configuration, and two seeds cannot see it

Three seeds each, same split and hyperparameters, differing only in
initialisation and batch order:

| | seeds | mean | sd |
|---|---|---|---|
| 4B | 36.7 / 34.7 / 34.7 | 35.4% | **1.2pp** |
| 9B | 44.9 / 44.9 / 38.8 | 42.9% | **3.5pp** |

The first two 9B seeds scored identically and read as *more* stable than the 4B.
The third tripled the spread. **Two seeds is not enough**, and noise cannot be
borrowed across configurations — this repo ran on one global ±5pp, which I first
replaced with the 4B's ±1.2pp, which was wrong in the same way.

### Cross-validation settles it: the gap is not significant, and McNemar always said so

Five folds, 4B, every non-benchmark alp14 command scored once by a model that
trained on none of it:

| fold | core | rate |
|---|---|---|
| f1 | 8/15 | 53.3% |
| f2 | 9/16 | 56.2% |
| f3 | 8/17 | 47.1% |
| f4 | 5/19 | **26.3%** |
| f5 | 9/14 | **64.3%** |

pooled **39/81 = 48.1%, SE 5.6pp**; fold sd 14.3pp.

**This rate is a compile rate.** `pass` in the eval files is `check_text` status:
Verus accepted the function with zero errors. Agreement with gold is a separate
axis (`semantic_equiv`) and runs about half as high — 20.4% / 22.4% where this
reports 36.7% / 44.9%. The distinction was never written next to the number
anywhere in this repo, which makes a syntax result very easy to read as a
correctness result. It is now printed above the table in `cv_summary.py`.

Attribution matters more than the spread. Expected fold sd from binomial sampling
alone, at ~16 commands per fold, is 12.5pp; observed exceeds it by 1.9pp, well
inside its own uncertainty. **These data do not show that command subsets differ
in difficulty** — the spread is a small test set, not which commands are in it.

What it does establish is the right error bar. Command sampling contributes
5.6pp; seeds contribute 1.2pp. Recomputing the 9B-over-4B gap of +7.5pp:

| against | |
|---|---|
| seed sd 2.6pp | 2.8 sd |
| **command SE 5.6pp** | **~1.3 sd — not significant** |

Which is exactly what McNemar reported at the outset (p = 0.454). **McNemar was
right; substituting seed noise for it produced two successively overstated claims,
both withdrawn.** Seed replicates cannot tell you whether a test set is
representative, and on 49 commands that is the dominant uncertainty.

**Aggregate stability is not per-command stability.** Two 9B seeds both scoring
22/49 disagree on 18 of 49 commands — nine each way, cancelling exactly. So a
claim like "the 9B handles command X" needs a second seed; only aggregate rates
are reproducible. Semantic verdicts hold up better: `RMI_VSMMU_CREATE` returns
`stronger` across five runs spanning two model sizes and both datasets.

**This does not overturn the McNemar results and the two are not substitutes.**
McNemar was non-significant because the disagreeing commands are few (16 for
4B vs 9B) — a power problem in the command set, not instability across seeds.
Together: the gaps are stable under retraining, and are not yet established
across a different set of commands. Only k-fold over command names fixes the
second; more seeds cannot.

Plausible reason the old split was noisier: 79 of its 98 evaluated commands were
in training, so performance there leans on memorisation, which is more sensitive
to initialisation than generalisation is.

### What the 42 failures actually are

Free, from data already collected: the folds score every core command exactly
once with a model that trained on none of it, so this is the cleanest failure
description available.

| | |
|---|---|
| type_mismatch | 18 |
| missing_symbol | 10 |
| wrong_arity | 7 |
| unbalanced_delimiter | 3 |
| other / parse / bad_field_access | 4 |

**All 42 are compilation failures. Not one is a spec that compiled and then
disagreed with gold.** On this axis the model's problem is entirely Verus
fluency, which is the same conclusion verus_rmm reached by a different route.

| family | compile rate | |
|---|---|---|
| `RMI_*` | 27/48 | 56.2% (SE 7.2pp) |
| `RSI_*` | 4/16 | 25.0% (SE 10.8pp) |
| `RMI_RTT_*` | 3/10 | 30.0% (SE 14.5pp) |
| `PSCI_*` | 5/7 | 71.4% (SE 17.1pp) |

Read the counts, not the ranking. At n = 7–16 the gaps are one to two SE, and
the fold analysis above already says these data cannot establish a command-subset
effect. What is worth noting is the direction: `RSI_*` and `RMI_RTT_*` are the
two families the benchmarks care about most.

One caution against generalising the repair fix. `wrong_arity` was 9 of 17
failures in the eac5 repair run and is 7 of 42 here. Same checkpoint, different
version, opposite dominant error — `type_mismatch` leads by a wide margin on
alp14. Fixing the feedback for arity is worth doing and will not move this.

---

## Benchmarks

| generator | rule_check (16) | false alarms | verus_rmm eac5 |
|---|---|---|---|
| gold | 16/16 | 0 | 4/4 |
| `sft3-2` 9B **+ preamble + repair ×3** | 8/8 eac5 | **0** | **3/4** |
| Claude + no-invent | 16/16 | 2 `RSI_FEATURES` | 1/4 |
| `sft3-2` 9B + preamble | 8/8 eac5 | **0** | 2/4 |
| `sft3-2` 9B | 16/16 | 2 `desc`* | **1/4** |
| Claude base | 14/16 | 2 `RSI_FEATURES` | 1/4 |
| `sft3-0` 4B | 11/16 | 0 | 0/4 |

The `sft3-2` eac5 cell read 0/4 until 2026-08-18 and was wrong; see below. The
preamble row is eac5-only because that run generated eac5 only.

\* SCOPE's own patch marks `desc` FP — a checker limitation. Claude's
`RSI_FEATURES` is a real miss: the table defines `value` and it wrote `true`.

**The repair claim is withdrawn. The preamble result replaces it.**

This document previously read: *one feedback round on `sft3-2` moved eac5 from
0/4 to 1/4, and that proves verus_rmm is compile-bound.* Re-scored today against
a validated gold control:

| `sft3-2` eac5 | verus_rmm TP | rule_check | false alarms |
|---|---|---|---|
| unrepaired | **1/4** | 8/8 | 1 `desc` |
| + 1 repair round | **1/4** | 8/8 | 1 `desc` |
| **+ training preamble** | **2/4** | 8/8 | **0** |

**The unrepaired baseline was 1/4 all along; the repair round changed nothing.**
Both score the same single item, `RSI_ATTESTATION_TOKEN_CONTINUE:dual_error`
(proof accepted). The recorded 0/4 was an error in this repo, not a result.

Restoring the preamble that training used adds `RMI_DATA_DESTROY:ripas` (proof
rejected) for 2/4, and separately removes the one rule_check false alarm. It is
the only intervention tried so far that moves this benchmark.

The **conclusion** the withdrawn evidence supported still holds, now on better
evidence: every non-detected item is `inconclusive`, meaning the function does
not compile and the obligation never runs. What changed is which lever works —
teaching the model the API surface it trained with, not showing it its own error.

Controls on the re-score, all four reproducing the committed record exactly:

| | verus_rmm eac5 | rule_check |
|---|---|---|
| gold | 4/4 TP, 6/6 FP | 8/8, 0 false alarms |
| `sft3-0` 4B | 0/4 | 6/8 eac5, 5/8 rel0 |

Gold matches `scores_eac5_gold.json` on the branch; the 4B matches every number
in `gpu-and-runs.md`. Only the 9B eac5 cell was wrong.

**How it was caught, which is the uncomfortable part.** It surfaced from
`gen4-9b` — a run submitted only because of the contamination false alarm below,
to re-establish a baseline that never needed re-establishing. Its output is
byte-identical to `gen3-9b` across all 41 commands (greedy decoding is
deterministic, so the pipeline reproduces exactly), and that identical artifact
scored 1/4 where the record said 0/4. **A mistaken alarm ran the replicate that
found a real error, and no deliberate check had.** Nothing in the process
deserves credit for this.

The two controls on the repair pass did hold. **SHRANK: 0 across all 41** — no
repair bought compilation by dropping constraints. And `RMI_RTT_READ_ENTRY`
still leaves `walk_level` unconstrained after repair: a pass optimising for
compilation could have filled in the undefined output and refuted the session's
main finding with its own tooling.

### The repair fix does work — it needed a working instrument to show it

`572e70f` (take identifiers from the error's quoted source, not just backticks)
was committed unvalidated. Rerun as `rep4-9b`, single-variable against `rep3-9b`:

| `sft3-2` eac5, verus_rmm | TP | detected |
|---|---|---|
| baseline, no repair | 1/4 | `RSI_ATTESTATION_TOKEN_CONTINUE` |
| repair 1 round, **old** feedback | 1/4 | same — no gain |
| repair 1 round, **fixed** feedback | **2/4** | **+ `RMI_DATA_DESTROY:ripas`** |
| **preamble**, no repair | **2/4** | **+ `RMI_DATA_DESTROY:ripas`** |
| gold | 4/4 | |

Two independent levers, the same one item unlocked. The preamble and the repair
fix are not obviously additive; `genpr-9b` tests the corner.

The new diagnostics say the loop is now mechanically sound, and that this is not
where the remaining failure lives:

| | `rep4-9b` |
|---|---|
| compile first try | 23/41 |
| compile after repair | 25/41 (`rep3-9b`: 24) |
| repairs attempted | 18 |
| declarations reached the prompt | **17 of 18** (2–6 lines each) |
| model returned a usable function | 18 of 18 |
| output text actually changed | 17 of 18 |
| SHRANK | **0** |
| failures converted to compiling | **2 of 18** |

So the model is being shown the declaration, is answering, and is not cheating —
and still fixes only two. The reason is visible in the class transitions:
`wrong_arity` goes 13 → 8, but the five it fixes surface as `bad_field_access`
(3) and `type_mismatch` (2) instead of passing. **Errors cascade, and one round
sees only the first one.** That makes rounds, not feedback quality, the next
variable worth moving.

### The preamble effect shrinks after decontamination, and is not significant here

`gpu-and-runs.md` records +22.5pp compiling at p = 0.004 for the 9B. That was
`sft2-*` on `dataset_clean`, 40 commands. On `sft3-*` / `dataset_bench`, 49
commands, paired on the same test set:

| | baseline | + preamble | net | McNemar exact |
|---|---|---|---|---|
| 4B | 18/49 (36.7%) | 21/49 (42.9%) | +3 | p = 0.375 |
| 9B | 22/49 (44.9%) | **27/49 (55.1%)** | +5 | p = 0.227 |

And on the axis that matters, the two models go opposite ways. Correctness is
`semantic_equiv` agreement with gold, excluding `PSCI_CPU_OFF` where gold is
itself vacuous and matching it is free:

| | compile | correct | `weaker` |
|---|---|---|---|
| 4B baseline | 18/49 (36.7%) | 10/49 (20.4%) | 1 |
| 4B + preamble | 21/49 (42.9%) | **9/49 (18.4%)** | **2** |
| 9B baseline | 22/49 (44.9%) | 11/49 (22.4%) | **0** |
| 9B + preamble | 27/49 (55.1%) | **15/49 (30.6%)** | 1 |

**The preamble lifts compilation on both models and converts it into correctness
only on the 9B.** On the 4B it is +3 compiling and −1 correct: three more specs
that Verus accepts, none of which agrees with gold, and one previously-correct
command (`RMI_VDEV_GET_INTERFACE_REPORT`) lost. `weaker` — the failure compile
checks structurally cannot see — goes 1 → 2 on the 4B and 0 → 1 on the 9B.

McNemar on correctness: 4B p = 1.000 (0 gained, 1 lost), 9B p = 0.219 (5 gained,
1 lost). Neither is significant, and the 9B's five gains are the only positive
evidence anywhere in this section.

So **"restore the preamble" is not a general recommendation.** It is a 9B result,
it costs that model the zero-`weaker` property this document called its only
defensible advantage, and on the 4B it buys compilation by producing more code
that compiles and means the wrong thing. That is precisely the trade this repo
refuses elsewhere — nothing may be trained on compile-success — and it shows up
here as an inference-time condition instead.

**Neither is significant**, and the effect is roughly half its previously
recorded size. The preamble is also not monotone — it breaks 1 command on the 4B
and 3 on the 9B, and `RMI_RTT_CREATE` moves in opposite directions on the two
models.

What can honestly be said: four separate measurements move the same way —
4B compile +3, 9B compile +5, verus_rmm 1/4 → 2/4, rule_check false alarms
1 → 0. Consistent direction across four is the argument; no single one of them
carries it, and the two that have p-values do not clear 0.05. Given how this
window went, that distinction is the whole point: a consistent direction is a
reason to keep the condition, not a result to report as established.

### The 2×2: the levers do not compose, and the TP column was hiding the story

`sft3-2`, eac5, all four cells:

| | TP (of 4) | FP fired (of 6) | compile first try | compile after repair |
|---|---|---|---|---|
| baseline | 1/4 | 0/6 | 23/41 | — |
| + repair ×1 | 2/4 | 0/6 | 23/41 | 25/41 |
| + repair ×3 | 2/4 | 3/6 | 23/41 | — |
| + preamble | 2/4 | 0/6 | 27/41 | — |
| + preamble + repair ×1 | 2/4 | 3/6 | 27/41 | 32/41 |
| **+ preamble + repair ×3** | **3/4** | **5/6** | 27/41 | — |
| gold | 4/4 | 6/6 | 41/41 | — |

**Rounds were the right next variable, but only together with the preamble.**
Three rounds without it stays at 2/4. Three rounds with it reaches 3/4 with a
single item left inconclusive — 8 of the 10 obligations now actually execute,
against 1 of 10 at baseline. The cascade reading holds: each round clears one
error class and exposes the next, so the loop needs both the declarations to fix
against and enough rounds to walk the chain.

This is the closest anything in this project has come to gold's (4/4, 6/6), and
it is entirely inference-time scaffolding — same checkpoint, same weights.

**But 3/4 is a benchmark number, and the correctness picture is weaker.** Same
two artifacts, `semantic_equiv` against eac5 gold, all 41 commands:

| eac5 | equivalent | compile_error | incomparable | stronger |
|---|---|---|---|---|
| baseline | 20/41 | 18 | 1 | 2 |
| + preamble + repair ×3 | **23/41** | **8** | **8** | 2 |

Compile errors fall 18 → 8, so ten specs start compiling. **Only three of those
ten become `equivalent`; seven become `incomparable`** — they now disagree with
gold in both directions where before they made no checkable claim at all. By the
"admits behaviour gold forbids" measure (`weaker` + `incomparable`) that is
1/41 → 8/41.

This is the 4B preamble trade again at larger scale, and it is the honest
qualification on the headline: the scaffolding converts *does-not-compile* into
*compiles*, and most of that conversion lands in *compiles and is wrong* rather
than *compiles and is right*.

Whether that is progress depends on the goal, and the two goals in this project
diverge here:

- **For bug-finding** it is straightforwardly good. An obligation that cannot run
  detects nothing; verus_rmm went 1/4 → 3/4 for exactly this reason, and a wrong
  spec that compiles is at least visible to the checker.
- **For faithful spec generation** it is close to neutral: +3 correct against
  +7 newly-wrong-and-checkable.

Neither `weaker` count moved (0 in both), which is the one guardrail that held —
nothing became a spec that silently permits what gold forbids in the pure sense.

`--with-preamble`'s help text and the claims table should be read with this
attached: the condition is a compile-rate and benchmark lever, and it has never
been shown to be a correctness lever on either model. On the 4B it was negative.

**At one round they do not compose.** Both single levers reach 2/4; together,
still 2/4. Read alone, that column says the combination bought nothing — and at
one round that reading is right. It stops being right at three.

**It bought a great deal.** Compilation goes 23 → 32 of 41, and the preamble
makes the repair loop three times more effective: it converts 5 of 14 failures
where the no-preamble run converted 2 of 18. Giving the model the API surface it
trained with is what makes its self-repair work — the two interact even though
the TP score does not show it.

**And the FP column is not a precision result.** The 0/6 in the first three rows
is not the generator being careful; it is the obligation never running because
the spec did not compile. As compilation rises the known-FP items start firing,
and gold fires all six — the benchmark's own note says a faithful generator
fails them exactly as gold does, so they measure the pipeline's ceiling rather
than generator quality. On both axes the combination moves toward gold's
profile, from (1/4, 0/6) to (2/4, 3/6) against gold's (4/4, 6/6).

The general lesson is the one this window keeps repeating in different clothes:
**`inconclusive` is not a score, and a metric computed over mostly-inconclusive
items is measuring whether the pipeline ran, not how well it did.** Three rows
of this table read as "0 false alarms" while nothing had executed.

rel0, preamble, no repair: **2/3 TP, 2/6 FP** — the 4B was 1/3. rule_check with
the preamble is 16/16 across both versions with one false alarm (`desc` on rel0,
none on eac5); without it, two.

**The 4B's two VERSION "misses" are a scoring artifact.** It read the prose that
defines `lower`/`higher` and encoded it, so nothing dangles and the check cannot
fire. Gold and the models that leave it blank are credited. The benchmark rewards
reading only the structured table.

---

## Negative results worth keeping

**Supplying gold's signature fixes signatures and nothing else.** Claude's
signature match goes 15% → 100%; verus_rmm stays 1/4. Two independent layers: the
signature layer blocks `semantic_equiv` comparison (35/41 uncomparable), the body
layer blocks compilation. Only the second bounds verus_rmm.

**One `stronger` verdict is gold being incomplete.** `RMI_VSMMU_CREATE` returns
`stronger` in five independent runs; gold carries a frame condition for `aidr` and
none for `idr[0..3]`, while the Footprint lists only `state` and `num_vsmmus`. The
models are right and are scored wrong. Screening all seven flagged two; reading
them held one — `RMI_REC_DESTROY` looked like a second and is not.

---

## Infrastructure

Three cluster profiles (`CLUSTER=boogiebonjour|turbox|research-common`), a local
Verus toolchain so correctness analysis no longer needs a long-lived pod, a
Verus-feedback repair pass, and a local driver for both benchmarks.

**Both clusters were oversold.** turbox shares its 32 GPUs with an arc-runners CI
fleet that routinely holds 25-28. research-common looks free at 45 GPUs but is 22
once tainted and cordoned nodes are excluded, and nothing schedules there at all —
Volcano reports `Unschedulable` with no reason and its queue objects are Forbidden
to read, so it needs an admin. On a shared cluster the binding constraint has been
CPU and memory, not GPU: two jobs sat Pending for 8-14 minutes purely because
resource requests were copied from a larger cluster's profile.

## Bugs fixed, all self-inflicted

AppleDouble entries in macOS-built archives (`tar tzf` hides them, Python's
tarfile does not); a failed run exiting 0 so k8s reported Complete with no
artifact; `score_benchmarks.sh` reporting a clean 0/0 and later a clean 0/4 on
empty input — the second produced a wrong conclusion before it was caught; a
PVC delete/recreate race that left a Job referencing a volume that did not exist.

---

## Open

Both of the first two items below closed before the window ended; they are kept
with their outcome rather than deleted, because the section was read once while
they were still stale.

- ~~`rep3-9b`, the repair pass, was still running.~~ **Closed:** it finished and
  moved verus_rmm 0/4 → 1/4, which is the "cannot compile" answer. The follow-up
  fix to the repair feedback (`572e70f`) is still unvalidated — see below.
- ~~9B seed evals (`ev3-9bs`) need resubmitting.~~ **Closed:** submitted and
  Succeeded; the 3.5pp 9B seed sd in this document is that run.
- The `--with-preamble` condition has never been run on `sft3-*`; it was worth
  +22.5pp compile rate on the 9B earlier and is likely the best configuration.

### Follow-up runs submitted 2026-08-18

Five jobs on turbox, all `sft3-*` on `dataset_bench` / prompt `v3.1`:

| job | mode | question |
|---|---|---|
| `rep4-9b` | gen, repair 1 | does the `572e70f` feedback fix beat `rep3-9b`'s 1/4? |
| `gen4-9b` | gen, repair 0 | reproducibility replicate of `gen3-9b` — see the note below |
| `genp-9b` | gen, preamble | is verus_rmm's compile bound liftable by the preamble? |
| `pre3-9b` | score, preamble | does the +22.5pp preamble effect survive decontamination? |
| `pre3-4b` | score, preamble | same, 4B, where it was p = 0.549 before |

`rep4-9b` is a single-variable A/B against `rep3-9b`: identical env, one code
change between them.

### A contamination that turned out not to exist

Reported here first as fact, then withdrawn within the hour. Recorded in full
because the withdrawal is the useful part.

`gen/sft3-2-final.tgz` holds 82 `.rs` files. I read the producing Job's
environment as `GEN_VERSIONS=eac5`, concluded the other 41 were stale files left
on the volume by an earlier run, and wrote it up as a contamination affecting
every eac5/rel0 benchmark number.

The Job's log says `[gen] eac5: 41 commands` and `[gen] rel0: 41 commands`,
`generated 82 spec files`, one upload. It produced all 82. The variable was
`GEN_VERSIONS="eac5 rel0"` all along; the command I inspected it with piped the
env through `tr ' ' '\n'`, which split the value on its space and showed me the
first half. **The formatting of my own query became the finding.**

This is the same failure as the four automated screens earlier in the window —
a derived number reported before one case was checked by hand — in a new
costume, and it is the seventh time. The rule was already written down. What it
missed is that a value read out of a tool is a derived number too.

The guard in `ba7b3d9` is kept, on a correct rationale: the PVC is per-Job but
outlives the pod, `backoffLimit` is 20, and resubmitting under the same name
reuses the volume — so a retry really can tar a previous attempt's partial
output. That path is real. The incident used to justify it was not, and both
the code comment and the commit's claim are corrected in the follow-up.

No published number changes, and `gen4-9b` above is now a plain reproducibility
replicate of `gen3-9b` rather than a cleanup — no run at 9B in this repo had
ever been repeated identically, so it is still worth its GPU minutes.

### Two notes for the benchmark's author

`ground_truth_eac5.json` has 10 items of which 4 are TP — that 4 is the
denominator in every `N/4` here, and the other 6 are documented non-
discriminating false positives. Separately, two distinct items share the id
`eac5:RMI_RTT_INIT_RIPAS:ripas` with opposite labels (one asserts
`old_walk.rtte.ripas == EMPTY`, the other `new_walk.rtte.ripas == RAM`). The
scorer iterates the list rather than keying on id, so both are scored and the
denominator is intact — but any consumer that does key on id would silently
drop one.

---

## A document-anchored check, and what it says about `incomparable`

`scripts/provenance.py` scores a spec against the **document** rather than against
gold, which is possible because each command section carries Failure / Success /
Footprint tables with stable IDs. Gold self-tests at 93.8% row coverage on eac5
and 96.3% on alp14 — those are measurements of the attributor, not of gold.

Applied to the `preamble + repair ×3` generation and to gold, same eac5 commands:

| | row coverage | clauses grounded | dangling outputs |
|---|---|---|---|
| gold | 437/466 (93.8%) | 96.7% | 9 |
| `sft3-2` + scaffold | **439/466 (94.2%)** | 95.5% | 9 |

**The model tracks the document as well as gold does** — while `semantic_equiv`
says the two agree on only 23 of 41 commands. Those are not contradictory, and
resolving them is the point of having both.

### The eight `incomparable` commands

For each, whether the model or gold covers document rows the other misses:

| command | doc rows | gold | model | |
|---|---|---|---|---|
| `RMI_DATA_CREATE_UNKNOWN` | 18 | 15 | 15 | same rows |
| `RMI_DATA_DESTROY` | 18 | 17 | 17 | same rows |
| `RMI_REC_DESTROY` | 9 | 9 | 9 | same rows |
| `RMI_RTT_CREATE` | 23 | 22 | 22 | same rows |
| `RMI_RTT_FOLD` | 22 | 21 | 21 | same rows |
| `RMI_RTT_INIT_RIPAS` | 18 | 18 | 18 | same rows |
| `RMI_RTT_READ_ENTRY` | 16 | 16 | 16 | same rows, **+4 ungrounded** |
| `RMI_RTT_UNMAP_UNPROTECTED` | 12 | 11 | 11 | same rows |

**Every one covers exactly the same document rows as gold.** So `incomparable`
here is not "the model missed something the document states" — both encode the
same set of stated conditions, and disagree on *how*.

**The limit that makes this weaker than it sounds:** coverage is row-level, and
two clauses can both cover row *X* while saying materially different things
about it. "Same rows" therefore means *the disagreement is below this check's
resolution*, not *the two are equally faithful*. Row-level provenance and
`semantic_equiv` are complementary and neither subsumes the other.

### The one command with a real signal

`RMI_RTT_READ_ENTRY` — the `walk_level` command — is the only one where the model
writes clauses matching no document row, and it writes four:

```rust
result.is_Err() ==> RttEntryFromDescriptor(new_s, desc).MemAttr == ...old_s...
                                                       .S2AP    == ...
                                                       .SH      == ...
                                                       .addr    == ...
```

Gold has zero ungrounded clauses here. The command's Footprint is empty — the
document says it changes nothing — so these four are *consistent* with the
document while being stated by none of its rows.

And both gold and the model leave `walk_level` dangling. **The document gap is
reproduced, not papered over**, now confirmed from a third independent direction
after the 12-cell prompt table and the rule-check benchmark.

### What this makes possible next

The natural instrument is clause-level rather than row-level: for each document
row, take gold's clause and the model's clause *for that row* and ask Z3 whether
they agree. That localises a disagreement to a row of the PDF instead of to a
whole command, and it is the version that could adjudicate the `stronger`
verdicts — the family that contains the one known case of gold being wrong.

---

## The preamble window hides half the API, and it explains four earlier results

Found by taking the clause-level tool to the eight `incomparable` commands and
then reading one disagreement by hand.

### What the disagreement actually was

`RMI_DATA_DESTROY`, success conditions. The model's clauses are *structurally
identical* to gold's. One token differs:

```rust
gold : RttWalk_(new_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).rtte.state == UNASSIGNED
model: RttWalk (new_s, rd, ipa)                           .rtte.state == UNASSIGNED
```

Both are declared in the preamble and both are uninterpreted, so Z3 can never
prove them equal:

```
line   75:  pub open spec fn RttWalk_(s, rd, addr, level) -> RmmRttWalkResult;
line  674:  pub open spec fn RttWalk (s, rd, addr)        -> RmmRttWalkResult;
```

**The model is shown the last 200 lines of a 683-line preamble — lines 484–683.**
`RttWalk_` is at line 75. It has never seen the function gold uses 147 times
across these commands, and it uses it zero times.

### The arity numbers say the same thing twice

| | `RttWalk` 4 args | `RttWalk` 3 args |
|---|---|---|
| no preamble shown | **254** | 0 |
| preamble tail shown | 91 | **144** |
| gold (`RttWalk_`) | 238 | 1 |

With nothing shown, the model writes the **arity it learned in training** under
the **only name it can recall** — which is literally `E0061: this function takes
3 arguments but 4 were supplied`, the dominant repair failure at 9 of 17. Shown
the tail, it switches to the three-argument function: it compiles, and it means
something else.

### Scale

| version | preamble | window | share of the API gold uses that is **invisible** |
|---|---|---|---|
| eac5 | 683 lines | 484–683 | 18 of 87 — **21%** |
| **alp14** | 1632 lines | 1433–1632 | 94 of 185 — **51%** |

alp14 is the version the 49-command held-out eval runs on. `AddrIsGranuleAligned`
and `AddrIsProtected` — the predicates in nearly every failure condition — are
outside the window.

### Four results this unifies

1. **`wrong_arity`**, 9 of 17 repair failures — the `RttWalk`/`RttWalk_` arity gap.
2. **`missing_symbol`**, 10 of 42 CV failures — depending on a symbol table that
   is not shown.
3. **Clause-level agreement 58% on failure conditions but 3% on success
   conditions** — success conditions are the ones that walk the RTT.
4. **"The preamble raises compilation and not correctness"** — measured three
   separate times as a correlation. Here is the mechanism: the tail supplies a
   function that compiles instead of the function that is meant.

The cross-validation compile rate of 48.1% was measured under this handicap.

### The fix, and its cost

`load_preamble(version, section_text=...)` selects declarations named in the
command's own document section, one transitive step through the types they
mention, plus constants and type definitions.

| | eac5 | alp14 |
|---|---|---|
| tail-200 coverage | 79% | **49%** |
| selected coverage | **98.1%** | **99.6%** |
| prompt size | 7.5k → 8.3k chars | 8.9k → 21.8k chars |

2.4× the prompt on alp14, which is the honest cost. Opt-in via
`--preamble-mode selected` / `PREAMBLE_MODE=selected`; the default stays the tail
because every published number was produced with it.

**Not yet measured.** Four jobs are running to find out whether fixing the window
moves anything: `gsel3-9b` (generation, eac5) and `sel3-9b` / `sel3-4b` (the
49-command held-out eval, where the hidden share is 51%). A mechanism this clean
still has to be shown to pay.
