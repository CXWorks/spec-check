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

Both models detect only the contradiction item; all three obligation items are
unscorable because the generated spec functions **do not compile**. Verified by
attribution: each generated function was run against the preamble with no proof
obligation attached, and failed on its own. GPT fails with `E0308 mismatched types`
and `E0425 cannot find RMI_ERROR_RTT_AUX`; Claude with `E0308` and `E0599 no method
spec_shl` (the `int << n` form Verus does not provide).

`inconclusive` is reported separately and never folded into `missed` — 1/4 detected
with 3 unscorable is a different claim from 1/4 with 3 missed.

**The two models are indistinguishable**, reproducing the rule-mode benchmark's
finding on independent evidence. The bottleneck differs, though: there they produced
compiling code and confabulated postconditions; here compilation is the barrier.
`repair_loop_verus.py` targets exactly this and would raise the scorable fraction
without changing the benchmark.

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
command with two error codes contradicts itself by construction. Converting
candidates into findings requires the ordering filter described under *Next steps*.
Raw output: `work/alp14_bench_wip/witness_sweep.json` (local scratch).

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

- **4 positives is small.** With both models at 1/4 it cannot separate them.
- **Compilation, not detection, is the current bottleneck** for generator scoring.
  Running the Verus-feedback repair loop before scoring is the obvious next step.
- **Growing the positive count** means running the ordering filter over the 97
  witness-sweep candidates: parse each command's failure-condition-ordering section,
  take its transitive closure, and keep only pairs no ordering edge covers. That is
  precisely what separates bug 4 (ordering omits the `pdev_align` vs `pdev_state`
  edge — real) from bug 6 (ordering states `[da_en] < [vdev_id]`; only the annotation
  dropped it — annotation defect, labelled FP in `rmm_bugs.rs`).
- Bugs 1–3 exist only on eac5/rel0; a benchmark tracking the current spec would need
  new findings, which is what the sweep plus ordering filter is for.
