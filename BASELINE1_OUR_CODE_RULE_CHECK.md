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
   and re-implement SCOPE's two checks against it:
   - **Dangling-output check**: for each declared output (excluding the
     `ReturnCode` field), does its variable name appear anywhere in our
     generated spec body?
   - **Footprint check**: for each `==`-clause under a `result.is_Ok()`/`is_Err()`
     implication, does the LHS match a declared output or a declared footprint
     value?

## Results

### Full flagged list (paper Table 7 style)

Same format as the SCOPE paper's Table 7 (ABI Name / Categorization / Verdict /
Description), applied here to the dangling-output check's 11 hits on **our own**
generated alp14 code (not SCOPE's own code, and not the spec text itself — see
caveat below). "Categorization" reuses the paper's `H(d)` label since this is
the dangling-output rule check; "Verdict" distinguishes the two confidence
tiers explained below (`Confirmed` = output missing from the generated
signature entirely, `Needs review` = present in signature but never
constrained in the body).

| ABI Name | Missing/unconstrained output(s) | Categorization | Verdict | Description |
|---|---|---|---|---|
| RMI_DATA_DESTROY | `data`, `top` | H(d) | Confirmed | both output params entirely absent from generated fn signature |
| RMI_FEATURES | `value` | H(d) | Confirmed | output param absent from generated fn signature |
| RMI_PDEV_AUX_COUNT | `aux_count` | H(d) | Confirmed | output param absent from generated fn signature |
| RMI_REC_AUX_COUNT | `aux_count` | H(d) | Confirmed | output param absent from generated fn signature |
| RMI_RTT_AUX_UNMAP_UNPROTECTED | `top` | H(d) | Confirmed | output param absent from generated fn signature |
| RSI_FEATURES | `value` | H(d) | Confirmed | output param absent from generated fn signature |
| RSI_MEM_SET_PERM_INDEX | `new_cookie` | H(d) | Confirmed | output param absent from generated fn signature |
| RSI_VSMMU_ACTIVATE | `new_base` | H(d) | Confirmed | output param absent from generated fn signature |
| RMI_PSMMU_IRQ_NOTIFY | `action`, `rd`, `vsmmu`, `msi_addr`, `msi_data` | H(d) | Needs review | present in signature, only pinned on failure path; SCOPE's own PDF extraction *also* has empty success conditions here — may be a PDF ambiguity, not just a generation miss |
| RMI_RTT_SET_S2AP | `rtt_tree` | H(d) | Needs review | present in signature but never constrained in body |
| RMI_VDEV_VALIDATE_MAPPING | `out_top` | H(d) | Needs review | present in signature but never constrained in body |

**This is a different kind of finding from "spec text self-contradiction"**
(what our `ensures false` sweep and SCOPE's `reason`-mode look for). These are
*generation-completeness* gaps — our LLM silently dropping or under-specifying
an output value — detected by re-using SCOPE's dangling-output *rule*, not a
logical-inconsistency proof. Whether the underlying alp14 spec text itself is
also self-contradictory for these 11 commands hasn't been checked here.

### Dangling-output check — 11 / 98 commands flagged, and these look real

Verified by hand (not just syntax mismatch): our generated spec functions split
into two failure patterns:

**Output parameter dropped entirely from the function signature** (8 commands) —
the LLM didn't just fail to constrain the value, it never declared it as a
parameter at all:

| Command | Missing output(s) |
|---|---|
| `RMI_DATA_DESTROY` | `data`, `top` |
| `RMI_FEATURES` | `value` |
| `RMI_PDEV_AUX_COUNT` | `aux_count` |
| `RMI_REC_AUX_COUNT` | `aux_count` |
| `RMI_RTT_AUX_UNMAP_UNPROTECTED` | `top` |
| `RSI_FEATURES` | `value` |
| `RSI_MEM_SET_PERM_INDEX` | `new_cookie` |
| `RSI_VSMMU_ACTIVATE` | `new_base` |

Example — `RMI_DATA_DESTROY` per the PDF has 3 outputs
(`result`, `data`, `top`), and SCOPE's own extraction shows success conditions
`data == walk.rtte.addr` and `top == walk_top`. Our generated signature is:
```rust
pub open spec fn rmi_data_destroy_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
```
— no `data` or `top` parameter at all, so those two output values are simply
absent from the formalization.

**Output parameter present in the signature but never constrained anywhere in
the body** (3 commands) — a subtler gap:

| Command | Unconstrained output(s) |
|---|---|
| `RMI_PSMMU_IRQ_NOTIFY` | `action`, `rd`, `vsmmu`, `msi_addr`, `msi_data` |
| `RMI_RTT_SET_S2AP` | `rtt_tree` |
| `RMI_VDEV_VALIDATE_MAPPING` | `out_top` |

Example — `RMI_PSMMU_IRQ_NOTIFY`'s generated body only pins these 5 values on
the *failure* path (`result != RMI_SUCCESS ==> X(new_s) == X(old_s)`, i.e.
"unchanged on failure") but never says what they equal on success. Notably,
SCOPE's own PDF table-extraction for this command *also* has an empty
success-conditions block — so this may not be purely an LLM mistake; it could
reflect the PDF itself not stating these success values in the structured
table SCOPE/we both parse from (worth a manual look at the PDF text for this
command before calling it a generation bug vs. a spec documentation gap).

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

The dangling-output check — the simpler, more syntax-robust of SCOPE's two
rule-based checks — surfaces 11 concrete gaps in our alp14 generation that are
independent of (and non-overlapping with) both SCOPE's original Table 7 bug
list and our own `ensures false` sweep's 2 confirmed RMM bugs. 8 of these are
clean, high-confidence findings (output silently dropped from the function
signature). The footprint check needs more normalization work before its
output can be trusted.
