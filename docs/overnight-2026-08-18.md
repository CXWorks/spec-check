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
compile check structurally cannot see.

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

---

## Benchmarks

| generator | rule_check (16) | false alarms | verus_rmm eac5 |
|---|---|---|---|
| gold | 16/16 | 0 | 4/4 |
| Claude + no-invent | 16/16 | 2 `RSI_FEATURES` | 1/4 |
| `sft3-2` 9B | 16/16 | 2 `desc`* | 0/4 |
| Claude base | 14/16 | 2 `RSI_FEATURES` | 1/4 |
| `sft3-0` 4B | 11/16 | 0 | 0/4 |

\* SCOPE's own patch marks `desc` FP — a checker limitation. Claude's
`RSI_FEATURES` is a real miss: the table defines `value` and it wrote `true`.

**verus_rmm is bounded by Verus syntax, and the repair pass proves it.** One
feedback round on `sft3-2` moved eac5 from 0/4 to 1/4 — not the 1/4 → 4/4 that
BENCHMARK_VERUS_RMM.md reports for Claude. The per-command log says why: of 39
commands 22 repaired clean, and **9 of the 16 failures are wrong_arity**,
concentrated exactly on the verus_rmm TP items (all four RTT commands fail, three
on arity).

The limit was the feedback, not the model. `preamble_decls` pulled identifiers
from backticks, and `E0061 this function takes 3 arguments but 4 were supplied`
never backticks the callee — it quotes the source line instead. So the model was
told its arity was wrong and never shown the declaration saying what the right
arity is. Fixed (identifiers now come from the quoted source too); **not yet
validated by a rerun**.

Two controls held. **SHRANK: 0 across all 39** — no repair bought compilation by
dropping constraints, which is the precondition for any of this meaning anything.
And `RMI_RTT_READ_ENTRY` still leaves `walk_level` unconstrained after repair: a
pass optimising for compilation could have filled in the undefined output and
refuted the session's main finding with its own tooling.

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

- `rep3-9b`, the repair pass, was still running. It decides whether verus_rmm's
  0/4 is "cannot find bugs" or "cannot compile".
- 9B seed evals (`ev3-9bs`) were dequeued to make room and need resubmitting.
- The `--with-preamble` condition has never been run on `sft3-*`; it was worth
  +22.5pp compile rate on the 9B earlier and is likely the best configuration.
