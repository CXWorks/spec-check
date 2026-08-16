# RMM Verus-verified spec-bug benchmark (eac5)

Every item here is a **machine-checked** spec defect: Verus either accepts a
contradiction proof or rejects a proof obligation the specification claims should
hold. No heuristic findings. The rule-mode (dangling-output) benchmark is separate,
at [`../rule_check_8bugs/`](../rule_check_8bugs/).

```
ground_truth_eac5.json     10 items: 4 TP (all discriminating) + 6 FP
run_bench.py               scorer, --version selects the spec version
scores_eac5_*.json         per-generator results
```

## Why eac5

Five distinct Verus-detectable bugs are known across RMM, and they are spread
unevenly across releases:

| # | Command | eac5 | rel0 | alp14 | Source |
|---|---|---|---|---|---|
| 1 | `RMI_DATA_DESTROY` | ✓ | ✓ | fixed | SCOPE reason mode |
| 2 | `RMI_RTT_DESTROY` | ✓ | ✓ | fixed | SCOPE reason mode |
| 3 | `RMI_RTT_INIT_RIPAS` | ✓ | fixed | fixed | SCOPE reason mode |
| 4 | `RMI_PDEV_STOP` | n/a | n/a | ✓ | `rmm_bugs.rs` bug 4 |
| 5 | `RSI_ATTESTATION_TOKEN_CONTINUE` | ✓ | ✓ | ✓ | `rmm_bugs.rs` bug 5 |

**eac5 carries 4 of the 5** — more than rel0 (3) or alp14 (2). Bug 4 cannot appear
before 1.1 (`rmi_pdev_stop` did not exist). Bug 5 was verified present in all three
versions during this work: ARM has not fixed it across three releases, which
`rmm_bugs.rs` did not record.

## The two item shapes

| Kind | Proof | Bug present when |
|---|---|---|
| `contradiction` | `requires spec(...), <witness> ensures false` | Verus **accepts** |
| `obligation` | `requires spec(...) { assert(P); }` | Verus **rejects** |

For an obligation, `P` is lifted from the spec's own output summary table. A true
positive means the command's normative conditions do not entail what the summary
claims — the gap SCOPE reports. A generator that invents the missing postcondition
makes `P` provable and *misses* the bug, which is exactly the failure mode observed
in the rule-mode benchmark.

Outcomes are `detected`, `missed`, and `inconclusive` (the generated spec does not
compile, or the obligation needs a parameter absent from its signature).
**Inconclusive is never folded into missed.**

## The 6 FP items do not discriminate

SCOPE labels them false positives because its obligation drops a guard the spec
actually states — e.g. `RMI_RTT_DESTROY`'s summary says "After execution:
UNASSIGNED" unconditionally, while the Success conditions give `UNASSIGNED_NS` when
`!AddrIsProtected`. A *faithful* generator therefore fails them exactly as the gold
oracle does (control: 6/6). They measure the pipeline's precision ceiling, not
generator quality, and should not be read as a per-generator score.

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

## Usage

```bash
# control — expect TP 4/4
python3 run_bench.py --version eac5 \
    --gen-dir ../../training-dataset/specs/eac5 --gen-pattern '{cmd}_spec.rs'

# a generator
python3 run_bench.py --version eac5 \
    --gen-dir ../../results/baseline1_general/gpt56sol/eac5 \
    --gen-pattern '{cmd}/generated.formatted.rs' --label gpt56sol
```

Requires Verus at `training/verus-x86-linux/` and rust toolchain `1.97.1`
(`LD_LIBRARY_PATH` is set by the harness).

## Results

| Generator | TP detected | inconclusive | FP fired |
|---|---|---|---|
| Gold oracle (control) | **4/4** | 0 | 6/6 |
| GPT `gpt-5.6-sol` (high) | 1/4 | 3 | 0/6 (all 6 inconclusive) |
| Claude Opus 5 (high) | 1/4 | 3 | 0/6 (all 6 inconclusive) |
| Claude Opus 5 (high) **+ Verus repair** | **4/4** | 0 | 6/6 |

Both models detect only the contradiction item (bug 5), which needs just the two
failure clauses — both got those right. All three obligation items are inconclusive
because the generated spec functions **do not compile**: verified by running each
generated function against the preamble with no proof obligation attached. GPT fails
with `E0308 mismatched types` and `E0425 cannot find RMI_ERROR_RTT_AUX`; Claude with
`E0308` and `E0599 no method spec_shl` (the `int << n` form Verus does not provide).

Unrepaired, this reproduces the rule-mode evaluation's finding: **the two models are
indistinguishable.** But the bottleneck differs — there they generated compiling code
and confabulated postconditions; here compilation itself is the barrier.

`repair.py` feeds the real Verus error back to the same model. One round fixed all six
failing commands, taking Claude from **1/4 to 4/4 — gold parity**. The repairs are
type-level only: the `==>` occurrence count is unchanged in every function, and the
diffs contain only the `RMI_ERROR_RTT(...)` payload fix and `1int << x` →
`(1u64 << x) as int`. Repaired output lives in `results/verus_repair/`, so raw
generator numbers stay intact.

GPT has no repaired row: codex hit its usage limit (resets 2026-08-20 23:22). Re-run
`python3 repair.py --model codex --src ...` afterwards.
