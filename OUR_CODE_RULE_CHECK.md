# Applying SCOPE's Rule-Based Checks to Our Own Generated alp14 Code

**Goal**: SCOPE's `--mode rule` (footprint + dangling-output check) found real
bugs on eac5/rel0 (see [BASELINE1_SCOPE_REPRODUCTION.md](BASELINE1_SCOPE_REPRODUCTION.md)).
Can the same checking logic catch bugs in **our own** Qwen-generated alp14
Verus code?

## Approach

Three independent checks against our own generated alp14 Verus code.
**Across all three, we found 5 real bugs in alp14**: 2 dangling outputs, 2
footprint-declaration gaps, and 1 logical inconsistency.

### 1. Dangling-output check

SCOPE's `--mode rule` doesn't take Verus code as input — it parses the spec
PDF into its own tables (outputs / failure / success conditions / footprint)
and checks those directly, so it can't be pointed at our `.rs` files.
[`training/scope_rule_check_ourcode.py`](training/scope_rule_check_ourcode.py)
instead reuses SCOPE's PDF-parsed tables (`scope --mode raw`, generator-
independent) and re-implements the check against our own code: for each
declared output, does its name appear anywhere in our generated spec body?
If not, flag it.

That alone only tells you "our code never mentions this name" — not *why*.
Every hit was manually checked against the actual alp14 PDF, page by page,
to tell apart a genuine spec gap (the PDF's own structured table never
defines the output either, for anyone) from a generation defect (the PDF
gives a formula and our model just dropped it).

### 2. Footprint check

SCOPE's footprint table declares which state each command is allowed to
modify. A naive substring match between the footprint values (written in
the spec's bound-variable style, e.g. `walk.rtte.state`) and our generated
code (which explicitly threads `old_s`/`new_s` through every call) is too
noisy — 59/98 commands flagged, mostly false positives.
[`training/footprint_check_normalized.py`](training/footprint_check_normalized.py)
fixes this by parsing both sides into a small expression AST, expanding the
spec's Context-table variable bindings, stripping state-threading
arguments, and comparing the two structurally instead of as text (full
details in the Results section below).

### 3. Z3 logical-inconsistency sweep

Unrelated to SCOPE's tables — reuses the "unsatisfiable precondition"
technique from `training/inconsistency_analysis.py` (originally run against
the gold specs), adapted in
[`inconsistency_analysis_model.py`](inconsistency_analysis_model.py) for our
own generated code. For each generated spec function it builds a proof
obligation `requires <spec_fn>(...) ensures false` and asks Verus/Z3 to
prove it; if Z3 succeeds, the spec's own preconditions are self-
contradictory. Only runs on commands that already pass basic Verus
verification, since it needs a compiled spec fn to build the obligation
against.

## Results

### Dangling-output check — 2 genuine spec gaps

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

### Footprint check — naive version too noisy, normalized version finds 2 real bugs

The naive substring-matching version flags 59/98 commands — mostly false
positives. Root cause: our generated code explicitly threads state through
every helper call (`RttWalk(new_s, RealmAt(new_s, rd), ...)`), while SCOPE's
PDF-extracted footprint text uses bound local variables from the spec's own
pseudocode (`RttWalk(realm, ...)` with `realm: RealmAt(rd)` defined once in a
Context table). A plain substring match between the two styles mismatches
constantly.

**Fix**: [`training/footprint_check_normalized.py`](training/footprint_check_normalized.py)
parses both sides into a small expression AST and compares them structurally
instead of as text:

1. Parses each command's *Context* table (bound var → defining expression,
   e.g. `walk: RttWalk(realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY)`)
   from the `scope --mode raw` dump — present in the dump but previously
   discarded by `scope_rule_check_ourcode.py`'s parser.
2. Expands each declared footprint value by substituting Context bindings
   recursively.
3. Canonicalizes both the expanded footprint value and our code's flagged
   LHS/RHS: strips the `old_s`/`new_s` state-threading argument from every
   call, drops `as TYPE` casts, and applies a small alias table (only
   relevant for eac5/rel0 naming drift, not alp14).
4. Recognizes that a footprint entry naming a whole record (`rtte:
   RttEntryAt(RttAt(walk.rtt_addr), entry_idx)`) also covers our code
   narrowing to one of its fields (`....rtte.state`) — this is the same
   entity spelled two different ways in SCOPE's own extraction, not a naming
   drift on our model's part.
5. Excludes `X(new_s, ...) == X(old_s, ...)`-shaped clauses entirely (same
   call/field shape, only the state argument differs) — these assert *no
   change*, so they can never be a footprint violation regardless of whether
   `X` is declared. This single rule accounts for about half the naive
   check's false positives (clauses generated by our model's own
   "unchanged unless modified" boilerplate).

Run on the 42 alp14 commands that pass Verus (see the Verus-compile-rate note
below — checking un-compiling code just adds noise from broken primitive
names on top of the noise this is trying to remove): naive flags 22 commands
(107 individual clauses); after normalization, 76 clauses turn out to be
false positives (naming style or the `X_new == X_old` non-claim above) and
**2 commands have a real, PDF-confirmed footprint gap** (a few more
candidates remain soft/unresolved — see the script's "still flagged" output).

#### `RMI_VSMMU_CREATE` — footprint omits 6 of the fields its own Success conditions set (PDF p.451–452, B4.3.65)

Success conditions (p.451–452) initialize six fields on the `vsmmu` object:

```
state    post: vsmmu.state == VSMMU_INACTIVE
realm    post: vsmmu.realm == rd
reg_base post: vsmmu.reg_base == params.reg_base
reg_top  post: vsmmu.reg_top == params.reg_top
aidr     post: vsmmu.aidr == params.aidr
idr      post: (vsmmu.idr[0] == params.idr[0] && ... [7 elements])
```

But the Footprint table (B4.3.65.4, p.452) only declares:

```
state       GranuleAt(vsmmu_ptr).state
num_vsmmus  realm.num_vsmmus
```

`GranuleAt(vsmmu_ptr).state` is the *Granule* bookkeeping state (delegated /
undelegated / etc.) — a different field from `vsmmu.state` (the VSMMU
object's own operational state). None of the six `vsmmu.*` fields the
Success conditions actually establish appear anywhere in the Footprint
table. This is a genuine footprint-declaration gap in the spec itself: the
table promises less than the command's own success conditions deliver.

#### `RMI_VSMMU_UNMAP` — same pattern (PDF p.459–460, B4.3.68)

Success conditions (p.459–460) include `state post: vsmmu.state ==
VSMMU_INACTIVE`, but the Footprint table (B4.3.68.4, p.460) only declares
`data_state: GranuleAt(walk.rtte.addr).state` and `rtte:
RttEntryAt(RttAt(walk.rtt_addr), entry_idx)` — `vsmmu`/`vsmmu.state` is
absent, same gap as `RMI_VSMMU_CREATE`.

A few softer candidates turned up during manual review but aren't counted
above: `RMI_REALM_DESTROY` looks like it has the same kind of gap
(`MecState`/`MecMembers` established in Success conditions but missing from
Footprint) but our extraction script's clause regex only matches simple
`result.is_Ok() ==> ...` guards and misses the compound-guarded form our
model happened to use here (`result.is_Ok() && <extra precondition> ==>
...`), so it was found by manual reading, not by the tool, and `RMI_FEATURES`
/ `RMI_REC_AUX_COUNT`'s remaining candidates are just collateral damage from
their already-documented dangling-output bugs (a hallucinated `.mem[]` read
and a nonsensical `== 0` placeholder standing in for the missing output
parameter), not new findings.

## Takeaway

This pipeline's two rule-mode checks (dangling-output + normalized footprint)
find **4 genuine, novel spec-documentation gaps in alp14**, each confirmed by
reading the PDF's own structured tables directly — the same shape of finding
as SCOPE's original Table 7 bugs on eac5/rel0:

| Command | Check | Gap |
|---|---|---|
| `RMI_RTT_SET_S2AP` | dangling-output | `rtt_tree` output never defined by any Failure/Success condition |
| `RMI_PSMMU_IRQ_NOTIFY` | dangling-output | all 5 outputs undefined — Success conditions section is empty |
| `RMI_VSMMU_CREATE` | footprint | Footprint table omits 6 `vsmmu.*` fields its own Success conditions set |
| `RMI_VSMMU_UNMAP` | footprint | Footprint table omits `vsmmu.state`, same gap as `RMI_VSMMU_CREATE` |

Every other candidate either check flagged along the way (9 more
dangling-output hits, dozens of footprint hits) turned out, on inspection,
to be a generation defect or a false positive of the checking method itself
— not a spec bug.

## Logical-inconsistency sweep (Z3 `ensures false`) — Qwen v5 finds 1 real bug

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
