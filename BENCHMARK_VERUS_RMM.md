# A Verus-Verified Spec-Bug Benchmark for ARM CCA RMM

Every item in this benchmark is **machine-checked**: Verus either accepts a
contradiction proof or rejects a proof obligation the specification itself claims
should hold. Heuristic findings are excluded — the rule-mode (dangling-output)
evaluation is a separate artifact, [`benchmark/rule_check_8bugs/`](benchmark/rule_check_8bugs/).

Benchmark and scorer: [`benchmark/verus_rmm/`](benchmark/verus_rmm/).

## Result

| Generator | TP detected (of 4) | Inconclusive | FP items fired (of 6) |
|---|---|---|---|
| **Gold oracle** (control) | **4/4** | 0 | 6/6 |
| GPT `gpt-5.6-sol` (high) | 1/4 | 3 | 0/6 — all 6 inconclusive |
| Claude Opus 5 (high) | 1/4 | 3 | 0/6 — all 6 inconclusive |
| Claude Opus 5 (high) **+ Verus repair** | **4/4** | 0 | 6/6 |

Both models detect only the contradiction item; all three obligation items are
unscorable because the generated spec functions **do not compile**. Verified by
attribution: each generated function was run against the preamble with no proof
obligation attached, and failed on its own. GPT fails with `E0308 mismatched types`
and `E0425 cannot find RMI_ERROR_RTT_AUX`; Claude with `E0308` and `E0599 no method
spec_shl` (the `int << n` form Verus does not provide).

`inconclusive` is reported separately and never folded into `missed` — 1/4 detected
with 3 unscorable is a different claim from 1/4 with 3 missed.

**Unrepaired, the two models are indistinguishable**, reproducing the rule-mode
benchmark's finding on independent evidence. But the bottleneck differs: there they
produced compiling code and confabulated postconditions; here compilation is the
barrier — and once it is removed, Claude reaches gold parity (see *Repair changes the
picture entirely* below). GPT could not be repaired for comparison because codex hit
its usage limit.

## Five Verus-detectable RMM bugs, and why the benchmark uses eac5

| # | Command | eac5 | rel0 | alp14 | Source |
|---|---|---|---|---|---|
| 1 | `RMI_DATA_DESTROY` | ✓ | ✓ | fixed | SCOPE reason mode |
| 2 | `RMI_RTT_DESTROY` (state unchanged) | ✓ | ✓ | fixed | SCOPE reason mode |
| 3 | `RMI_RTT_INIT_RIPAS` | ✓ | fixed | fixed | SCOPE reason mode |
| 4 | `RMI_PDEV_STOP` | n/a | n/a | ✓ | `training/rmm_bugs.rs` bug 4 |
| 5 | `RSI_ATTESTATION_TOKEN_CONTINUE` | ✓ | ✓ | ✓ | `training/rmm_bugs.rs` bug 5 |

Deduped by `(command, property)`; no command overlaps between the two sources.
**eac5 carries 4 of the 5** — more than rel0 (3) or alp14 (2), which is why the
benchmark targets it. Bug 4 cannot exist before 1.1 (`rmi_pdev_stop` was added with
device assignment). Bugs 1–3 were fixed by ARM in the 1.1 series, verified by
comparing generated proof obligations: `rmi_data_destroy`'s eac5 constraint
`ripas == EMPTY || ripas == RAM` becomes the tautology `ripas != RAM || ripas == RAM`
in alp14, and `rmi_rtt_destroy`'s `new.state == old.state` becomes the concrete
`new.state == UNASSIGNED`.

## Two new findings from this work

Proofs in [`benchmark/verus_rmm/new_findings.rs`](benchmark/verus_rmm/new_findings.rs)
(Verus: 1 verified, 0 errors).

**1. Bug 5 is not alp14-specific.** `rmm_bugs.rs` records it as an alp14 finding.
Re-running its witness against the eac5 and rel0 gold annotations shows the
contradiction holds in all three released versions — ARM has not fixed it across
three releases.

**2. `RMI_RTT_DESTROY` summary table vs Success conditions (alp14, borderline).**
The output summary table (§B4.3.39.1.3) states *"After execution: UNASSIGNED and
RIPAS is DESTROYED"* unconditionally, while the Success conditions (§B4.3.39.3) give
`UNASSIGNED_NS` when `!AddrIsProtected`. For a successful destroy on an unprotected
IPA the two demand different states; the contradiction is machine-checked. Recorded
as **borderline** rather than confirmed because it depends on reading the summary
table as normative and unconditional — a reviewer could argue the row illustrates the
protected case. Structurally it is the same shape as the confirmed SDEI table-vs-text
bug in `spec_bug_report.md`.

## The existing automated sweep cannot find this bug class

`training/inconsistency_analysis_rmm.py` asks whether a spec function is unsatisfiable
**for all inputs**. Both confirmed bugs are only unsatisfiable **under a witness**.
On the gold `rmi_pdev_stop_spec`:

| Query | Verdict |
|---|---|
| `requires spec(...) ensures false` — what the sweep asks | rejected → reports "consistent" |
| same + `!AddrIsGranuleAligned && state == PDEV_STOPPING` | **accepted** → contradiction |

The sweep classifies a published bug as consistent. This explains its reported
"0 new inconsistencies across 89 alp14 gold specs": it is structurally blind to the
pattern, and the two known bugs were found by inspection, not by that tool.

A witness sweep over alp14 (every pair of failure conditions with differing error
codes, plus a vacuity probe so mutually-exclusive preconditions do not verify
`ensures false` trivially) rediscovers both known bugs and flags **97 deduped
candidates across 40 of 98 commands**. That figure is an **upper bound, not a bug
count**: the gold annotations transcribe failure conditions as unguarded implications
and never encode the spec's failure-condition-ordering section, so nearly every
command with two error codes contradicts itself by construction. The obvious way to
convert candidates into findings — filtering by the failure-condition-ordering
section — was implemented and does not work; see *Limitations and next steps*.
Candidates: `benchmark/verus_rmm/evidence/alp14_witness_sweep_candidates.json`.

## Benchmark construction

Two item shapes, because the two bug sources have different logical form:

| Kind | Proof | Bug present when |
|---|---|---|
| `contradiction` | `requires spec(...), <witness> ensures false` | Verus **accepts** |
| `obligation` | `requires spec(...) { assert(P); }` | Verus **rejects** |

For an obligation, `P` is lifted from the spec's own summary table; a true positive
means the normative conditions do not entail what the summary claims. A generator
that invents the missing postcondition makes `P` provable and *misses* the bug —
the same confabulation failure mode the rule-mode benchmark measures.

**The 6 FP items do not discriminate between generators.** SCOPE labels them false
positives because its obligation drops a guard the spec actually states — e.g.
`RMI_RTT_DESTROY`'s summary omits the `AddrIsProtected` guard that the Success
conditions apply. A faithful generator therefore fails them exactly as the gold
oracle does (control: 6/6). They bound the pipeline's precision, and must not be read
as a per-generator score. The 4 TP items are the discriminating set.

## Cross-version replication: rel0

The same construction applied to rel0 (`ground_truth_rel0.json`, 9 items: 3 TP + 6 FP —
`RMI_RTT_INIT_RIPAS` was fixed between eac5 and rel0, and `RMI_PDEV_STOP` does not exist
before 1.1) reproduces every eac5 conclusion on independent data:

| Generator | TP detected (of 3) | Inconclusive | FP fired |
|---|---|---|---|
| Gold oracle (control) | **3/3** | 0 | 6/6 |
| GPT `gpt-5.6-sol` (high) | 1/3 | 2 | 0/6 |
| Claude Opus 5 (high) | 1/3 | 2 | 0/6 |
| Claude Opus 5 **+ repair** | **3/3** | 0 | 6/6 |

Identical pattern: the two models tie unrepaired, both are limited by compilation rather
than detection, and one repair round restores gold parity. All six repaired functions
preserve their `==>` occurrence counts exactly (9/10/15/13/11/15).

## Validity controls

1. **Toolchain parity**: `training/rmm_bugs.rs` (15 verified, 0 errors) and
   `training/spec_bugs.rs` (10 verified, 0 errors) still check out under the locally
   installed Verus 0.2026.08.09, so it agrees with the machine that produced the
   published reports.
2. **Gold-oracle control**: 4/4 TP detected. Any generator deviation is attributable
   to the generator, not the harness.
3. **Attribution of failures**: every `inconclusive` was confirmed by compiling the
   generated function alone, without a proof obligation.
4. **Determinism**: two consecutive harness runs give identical outcomes on all 10
   items.
5. Ground truth is derived from SCOPE's own labelling patch and from machine-checked
   proofs — no item is labelled from the paper or from memory.

## Limitations and next steps

- **4 positives is small.** With Claude at gold parity after repair, the benchmark
  cannot rank a strong generator any finer than "matches the reference".
- **Compilation was the bottleneck, and it is now removable.** One round of
  Verus-feedback repair took Claude from 1/4 to 4/4. Generator comparisons should
  report both the raw and the repaired configuration; raw alone measures Verus syntax
  fluency more than bug-finding ability.
- **GPT is unmeasured in the repaired configuration** — codex quota resets
  2026-08-20 23:22. Re-run `repair.py --model codex` after that for the missing row.
- **Growing the positive count is harder than it first appears.** The obvious filter —
  parse each command's failure-condition-ordering section, close it transitively, and
  keep only pairs no edge covers — was implemented and **does not discriminate**. It
  classifies bug 4's pair correctly as uncovered and bug 6's as covered
  (`[da_en] < [vdev_id]`), but across alp14 it leaves **1,248 of 1,457 differing-code
  pairs uncovered**: ordering covers only 209, just 3 commands are fully covered, and
  11 commands have no ordering section at all. "Textually unordered" is therefore the
  norm, not a defect signal, and cannot be the criterion.

  What actually distinguishes bug 4 is not in the text. Below the two `<` relations,
  §B4.3.20.2.1 carries a *diagram* laying the conditions in tiers
  (`da_supp` → `{pdev_bound, pdev_align, pdev_gran_state}` → `{num_vdevs, pdev_state}`).
  The diagram implies `pdev_align < pdev_state`; the textual relations never state it.
  The bug is the gap between diagram and text — which the pdftotext extraction
  flattens, so recovering it needs the diagram's tier structure, not the `<` lines.
  Until that is solved, treat the witness sweep's 97 candidates as a *review queue*
  requiring human judgment, not an automated bug list.
- Bugs 1–3 exist only on eac5/rel0; a benchmark tracking the current spec would need
  new findings, which is what the sweep plus ordering filter is for.

## Repair changes the picture entirely

The three unscorable TP items were unscorable for one mechanical reason: `RMI_ERROR_RTT`
is a payload-carrying variant (`RMI_ERROR_RTT(int)`) and both models wrote it bare. The
gold annotation writes `RMI_ERROR_RTT(RttWalk(...).level as int)`.

Feeding the real Verus error back to the same model
([`benchmark/verus_rmm/repair.py`](benchmark/verus_rmm/repair.py), the project's existing
repair-loop idea with a CLI backend) fixed all six failing commands **in one round each**,
taking Claude Opus 5 from **1/4 to 4/4 — identical to the gold oracle**, with all 10 items
scorable.

The repairs are type-level only, verified two ways: the `==>` occurrence count is
unchanged in all six functions (14/9/10/11/13/15 before and after), and the diffs contain
only the payload fix and `1int << x` → `(1u64 << x) as int` (Verus has no shift on the
mathematical `int`). No condition was added, removed or weakened.

So the earlier "1/4" is a measurement of **Verus syntax fluency, not bug-finding ability**.
Once the code compiles, Claude finds every bug the hand-written reference does. Repaired
output is kept separately (`results/verus_repair/`) so the raw generator numbers stay
intact — "model + repair" is a different configuration, reported as its own row.

**GPT could not be measured this way**: codex hit its usage limit during the repair pass
(resets 2026-08-20 23:22). The quota detection stopped cleanly with a resume hint rather
than burning the remaining calls, and the run is resumable with the same command line.
