# Applying SCOPE's Rule-Based Checks to Our Own Generated alp14 Code

**Goal**: SCOPE's `--mode rule` (footprint + dangling-output check) found real bugs
on eac5/rel0 (see [BASELINE1_SCOPE_REPRODUCTION.md](BASELINE1_SCOPE_REPRODUCTION.md)).
Question: can the same *checking logic* catch bugs in **our own** Qwen-generated
alp14 Verus code, instead of SCOPE's own PDF-derived code?

## Why `scope --mode rule` can't be pointed at our code directly

SCOPE's `--mode rule` doesn't take Verus code as input at all. It parses the spec
PDF into its own structured tables (params / outputs / failure conditions /
success conditions / footprint) and runs the checks on that internal
representation — regardless of which downstream tool (SCOPE's own generator, or
ours) would eventually turn those tables into Verus code. Pointing
`scope --target alp14 --mode rule` at alp14 just re-runs SCOPE's own PDF parsing;
it never touches our generated `.rs` files.

## Approach

The outputs/footprint tables SCOPE extracts are spec-derived, generator-
independent metadata, so they can be reused. Script:
[`training/scope_rule_check_ourcode.py`](training/scope_rule_check_ourcode.py).

1. `scope --target alp14 --input-type pdf --mode raw > scope/alp14_raw.txt` —
   dumps SCOPE's parsed tables (outputs, failure/success conditions, footprint)
   for all 98 alp14 commands without generating any Verus code.
2. Parse that dump into `{cmd_name: {outputs, footprints, ...}}`.
3. For each command, read our own generated spec function
   (`results/ab_test_qwen_v3retrained/v3_qwen/alp14/<cmd>/generated.formatted.rs`)
   and check: for each declared output (excluding the `ReturnCode` field), does
   its variable name appear anywhere in our generated spec body? If not, flag
   it as a **dangling output**.

**Important**: step 3 only tells you "our generated code never mentions this
output name" — it says nothing about *why*. That requires a fourth, manual
step: read the spec PDF's own structured `Success conditions` table
(`Bx.y.z.3` sections) for the flagged command and check whether *it* ever
defines the output either. Skipping this step conflates two very different
findings:

- **Genuine spec gap**: the PDF's own structured table never defines the
  output — not our model's fault, nobody could translate something that isn't
  there. This is the same shape of finding as SCOPE's original Table 7 bugs
  (e.g. `RMI_RTT_READ_ENTRY`/`walk_level` on eac5/rel0).
- **Generation defect**: the PDF's table clearly defines the output with a
  formula, but our model dropped the parameter from the signature anyway.
  This is a translation failure, unrelated to spec quality.

All 11 dangling-output hits below were checked against the actual alp14 PDF
(`scope/DEN0137_1.1-alp14_rmm-arch_external.pdf`) page by page to tell these
apart.

## Results

### Dangling-output check — 11 / 98 commands flagged

**2 are genuine spec gaps** — the PDF's own structured Success/Failure
conditions tables never define the output, for anyone:

#### `RMI_RTT_SET_S2AP` — output `rtt_tree` (PDF p.395–397, section B4.3.45)

Output values table (p.395) declares three outputs: `result`, `out_top`,
`rtt_tree`. The only place `rtt_tree` is explained at all is a free-text
sentence directly under that table:

> If *result* is RMI_ERROR_RTT or RMI_ERROR_RTT_AUX then the following are true:
> • *out_top* is the IPA of the RTTE at which the base alignment check failed.
> • *rtt_tree* is the index of the RTT in which the base alignment check failed.

That's narrative prose, not a structured condition. The actual structured
tables never pin down a value for it:

- **Failure conditions** (p.396, 13 conditions: `rd_align`, `rd_bound`,
  `rd_state`, `rec_align`, `rec_bound`, `rec_gran_state`, `rec_state`,
  `rec_owner`, `size_valid`, `base_bound`, `top_bound`, `top_gran_align`,
  `base_align_pri`, `base_align_aux`) — every single `post:` is
  `ResultEqual(result, ...)`. None mentions `rtt_tree`.
- **Success conditions** (p.397):
  ```
  ID          Condition
  s2ap_addr   post: rec.s2ap_addr == out_top
  ```
  Only defines `out_top`. `rtt_tree` does not appear anywhere in this table.

So `rtt_tree` is declared as an output but never formally defined by any
`pre:`/`post:` condition in the whole command — the same pattern as SCOPE's
own `RMI_RTT_READ_ENTRY`/`walk_level` finding on eac5/rel0 (table exists,
output name just never shows up in it). Our generated code not defining
`rtt_tree` is therefore **not a translation mistake** — the spec gives no
formula to translate.

#### `RMI_PSMMU_IRQ_NOTIFY` — outputs `action`, `rd`, `vsmmu`, `msi_addr`, `msi_data` (PDF p.333, section B4.3.28)

Output values table declares all 5 fields. Directly below it, the Success
conditions section reads, in full:

> **Success conditions**
> The RMI_PSMMU_IRQ_NOTIFY command does not have any success conditions.

The structured table is completely empty — the same pattern as `PSCI_VERSION`
on eac5 (`"does not have any success conditions"`). None of the 5 success-path
output values are defined anywhere in the spec text. Our generated code
leaving them unconstrained on the success path is, again, **not a translation
mistake** — there is nothing in the spec to constrain them to.

### The other 9 are generation defects, not spec gaps

Checked each one's structured Success conditions table directly; in every
case the spec gives an explicit formula and our model simply dropped the
output parameter from the generated function signature entirely:

| Command | Missing output(s) | What the PDF's structured Success conditions table actually says | PDF page |
|---|---|---|---|
| `RMI_DATA_DESTROY` | `data`, `top` | `data post: data == walk.rtte.addr`; `top post: top == walk_top` | p.293 |
| `RMI_FEATURES` | `value` | `value post: value == RmiFeatureRegisterEncode(index)` | p.294 |
| `RMI_PDEV_AUX_COUNT` | `aux_count` | `aux_count post: aux_count == PdevAuxCount(flags)` | p.304 |
| `RMI_REC_AUX_COUNT` | `aux_count` | `aux_count post: aux_count == RecAuxCount(rd)` | p.345 |
| `RMI_RTT_AUX_UNMAP_UNPROTECTED` | `top` | `top post: top == walk_top` | p.373 |
| `RSI_FEATURES` | `value` | `value post: value == RsiFeatureRegisterEncode(realm, index)` | p.503 |
| `RSI_MEM_SET_PERM_INDEX` | `new_cookie` | `new_cookie post: New cookie is generated` (vague, but the output is at least named/addressed in the table — not silently absent like the two gaps above) | p.516 |
| `RSI_VSMMU_ACTIVATE` | `new_base` | Appears repeatedly as a bound in the `ripas`/`start`/`complete` conditions (e.g. `new_base != vsmmu.reg_top`, `new_base == vsmmu.reg_top`) — referenced throughout the table, just not extracted by our model | p.540 |
| `RMI_VDEV_VALIDATE_MAPPING` | `out_top` | `out_top post: out_top == MinAddress(top, walk_top_pre)` | p.447 |

Example — `RMI_DATA_DESTROY` per the PDF has 3 outputs
(`result`, `data`, `top`) with clear formulas as shown above. Our generated
signature is:
```rust
pub open spec fn rmi_data_destroy_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
```
— no `data` or `top` parameter at all, so those two output values are simply
absent from the formalization despite the spec spelling out exactly what they
should equal.

### Footprint check — not usable as implemented (too noisy)

59 / 98 commands got flagged — clearly mostly false positives, not real bugs.
Root cause: our generated code explicitly threads state through every helper
call (`RttWalk(new_s, RealmAt(new_s, rd), ...)`), while SCOPE's PDF-extracted
text uses bound local variables from the spec's own pseudocode
(`RttWalk(realm, ...)` with `realm: RealmAt(rd)` defined once). A naive
substring match between the two styles mismatches constantly. Would need much
deeper normalization (resolving our generated expressions back to SCOPE's
bound-variable form) to be a trustworthy signal — not attempted here.

## Takeaway

Of the 11 commands the dangling-output check flags in our alp14 generation:

- **2 are genuine spec-documentation gaps** — `RMI_RTT_SET_S2AP` (`rtt_tree`)
  and `RMI_PSMMU_IRQ_NOTIFY` (5 outputs) — where the PDF's own structured
  Success/Failure conditions tables never define the output for anyone, the
  same shape of finding as SCOPE's original Table 7 bugs on eac5/rel0. These
  are the closest thing to a *novel*, alp14-specific instance of a SCOPE-style
  rule-mode bug found by this pipeline.
- **9 are generation defects**, not spec bugs: the PDF clearly gives a formula
  for the output, but our model dropped the parameter from the signature
  during translation.

The footprint check needs more normalization work before its output can be
trusted (see above).

## A different check: logical-inconsistency sweep (Z3 `ensures false`) — Qwen v5

This is a separate check method from the SCOPE rule-mode checks above — it
doesn't use SCOPE's PDF-derived tables at all. Instead it reuses the
"unsatisfiable precondition" technique from
[`training/inconsistency_analysis.py`](training/inconsistency_analysis.py) /
[`training/BUG_REPORT.md`](training/BUG_REPORT.md) (originally run against the
gold RMM specs), adapted in
[`inconsistency_analysis_model.py`](inconsistency_analysis_model.py) to work
against our own model-generated specs in their per-command result-directory
layout. For each generated spec function, it builds:

```rust
proof fn check_inconsistency_<name>(...)
    requires <name>(...)
    ensures false
{}
```

and asks Verus/Z3 to verify it. If it succeeds (0 errors), the spec's own
preconditions are self-contradictory — flagged **INCONSISTENT**.

**Run:** `results/ab_test_qwen_v5/v3_qwen/alp14` (`item_split_v3_e2_best`,
5 rounds of Verus-feedback self-repair, final state =
`v5_genuine_round5_repaired.json`, 53/98 passing Verus). Only the 53 commands
that already pass basic Verus verification can be checked this way — a spec
that doesn't even compile has nothing for Z3 to reason about.

| Checked (Verus-passing) | INCONSISTENT | consistent | trivial body (skipped) |
|---|---|---|---|
| 53 | **1** | 50 | 2 (`psci_cpu_off`, `psci_cpu_suspend`) |

### `psci_affinity_info` — bare `&&` forces `result` to two different values at once

```rust
&& (result == PSCI_SUCCESS && RecFromMpidr(old_s, target_affinity).flags.runnable == RUNNABLE)
&& (result == PSCI_OFF && RecFromMpidr(old_s, target_affinity).flags.runnable == NOT_RUNNABLE)
```

Both conjuncts are unconditional (no `==>` guard), so the spec requires
`result == PSCI_SUCCESS` **and** `result == PSCI_OFF` to hold simultaneously —
impossible, since the two are different constants. Z3 confirmed this by
verifying `requires psci_affinity_info_spec(...) ensures false` with 0 errors.
Same root-cause shape as Bug 1 in `training/BUG_REPORT.md` (conditions that
should have been `==>`-guarded branches got written as top-level `&&`
conjuncts instead) — this failure mode has now shown up in two independent
generations of the model.

The other 45 commands that fail basic Verus verification in this run weren't
covered by this sweep (no compiled spec fn to build the proof obligation
against); GPT's comparable run (`results/ab_test_gpt/v3_gpt/alp14`, 65/98
Verus-passing) was checked the same way and came back 0/65 inconsistent.
