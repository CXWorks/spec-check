# Bug Report: Logical Inconsistencies in LLM-Generated PSCI Formal Specifications

**Date:** 2026-04-21
**Spec:** ARM Power State Coordination Interface (PSCI), DEN0022F.b v1.3
**Generator:** spec-gen pipeline — Qwen3-4B fine-tuned on RMM spec, applied zero-shot to PSCI
**Output file:** `psci_generated_clean.rs` (post-processed from `psci_generated.rs`)
**Verification tool:** Verus 0.2026.04.12 + Z3 v4.12.5

All four bugs were confirmed by Z3: `proof fn` lemmas with `ensures false` that
**successfully verify** demonstrate the spec is unsatisfiable or vacuous under the
stated conditions.  Final Verus run: **54 verified, 0 errors**.

---

## Bug 1 — `affinity_info_spec`: Error conditions conjuncted instead of disjuncted

### Severity
**Critical** — the spec is **unsatisfiable** whenever the target affinity is valid and
the call returns an error.  Any property that assumes a valid error return from
`AFFINITY_INFO` will be unprovable.

### Location
`psci_generated_clean.rs`, function `affinity_info_spec`, lines 208–221
Source: `psci_generated.rs`, lines 141–159 (model output)

### Description
The PSCI spec permits `AFFINITY_INFO` to return several different error codes
(`PSCI_INVALID_PARAMETERS`, `PSCI_DISABLED`, `PSCI_ALREADY_ON`, `PSCI_ON_PENDING`,
`PSCI_DENIED`, `PSCI_NOT_SUPPORTED`) depending on conditions.  The model generated a
separate implication for each error code and connected them all with `&&`:

```
// Generated (BUGGY):
CpuIsValid(s, target_affinity) ==> (result.is_Err() ==> code == PSCI_NOT_PRESENT || code == PSCI_DISABLED)
&& CpuIsValid(s, target_affinity) ==> (result.is_Err() ==> code == PSCI_ALREADY_ON)    // ← separate &&
&& CpuIsValid(s, target_affinity) ==> (result.is_Err() ==> code == PSCI_ON_PENDING)    // ← separate &&
&& CpuIsValid(s, target_affinity) ==> (result.is_Err() ==> code == PSCI_DENIED)        // ← separate &&
&& CpuIsValid(s, target_affinity) ==> (result.is_Err() ==> code == PSCI_NOT_SUPPORTED) // ← separate &&
```

When `CpuIsValid` is true and `result.is_Err()`, all conjuncts fire simultaneously,
requiring `code == PSCI_ALREADY_ON (-4)` **AND** `code == PSCI_ON_PENDING (-5)` **AND**
`code == PSCI_DENIED (-3)` **AND** `code == PSCI_NOT_SUPPORTED (-1)` — all different
integer constants.  This is arithmetically impossible.

### Root cause
The model produced one implication per error case instead of a single implication with
a disjunction of permitted error codes.  Each case should have been a `||`-branch inside
one consequent, not a separate `&&`-conjunct at the top level.

### Correct form
```rust
// Correct:
CpuIsValid(s, target_affinity) ==> (result.is_Err() ==> (
    code == PSCI_NOT_PRESENT
    || code == PSCI_DISABLED
    || code == PSCI_ALREADY_ON
    || code == PSCI_ON_PENDING
    || code == PSCI_DENIED
    || code == PSCI_NOT_SUPPORTED
))
```

### Proof witness (Z3-verified)
```rust
proof fn bug1_affinity_info_error_conjunction(
    s: S,
    target_affinity: Bits64,
    lowest_affinity_level: Int32,
    result: Result<(), PsciCode>,
    state: AffinityInstanceState,
    old_state: AffinityInstanceState,
    new_state: AffinityInstanceState,
)
    requires
        CpuIsValid(s, target_affinity),
        result.is_Err(),
        affinity_info_spec(s, target_affinity, lowest_affinity_level, result, state, old_state, new_state),
    ensures
        false,  // INCONSISTENCY CONFIRMED BY Z3
{
    assert(result.get_Err_0().as_int() == PSCI_ALREADY_ON);   // from conjunct 3: -4
    assert(result.get_Err_0().as_int() == PSCI_ON_PENDING);   // from conjunct 4: -5
    assert(PSCI_ALREADY_ON != PSCI_ON_PENDING);               // -4 != -5
}
```

---

## Bug 2 — `psci_cpu_default_suspend_spec`: Non-exclusive antecedents map to contradictory result codes

### Severity
**Critical** — the spec is **unsatisfiable** in any state where a CPU is running at a
non-secure address (a common valid system configuration).

### Location
`psci_generated_clean.rs`, function `psci_cpu_default_suspend_spec`, lines 230–233
Source: `psci_generated.rs`, lines 160–180 (model output)

### Description
The spec contains two implications that can both be triggered simultaneously:

```rust
// Generated (BUGGY):
(CpuIsOn(old_s, entry_point as Bits64)    ==> result == PSCI_SUCCESS)             // implication A
&& (AddrIsNonSecure(old_s, entry_point)   ==> result == PSCI_INVALID_PARAMETERS)  // implication B
```

A CPU can be on (`CpuIsOn` = true) while its entry point resides in non-secure memory
(`AddrIsNonSecure` = true).  When both conditions hold:

- Implication A: `result == PSCI_SUCCESS` → `result == 0`
- Implication B: `result == PSCI_INVALID_PARAMETERS` → `result == -2`

These force `0 == -2`, which is false.

### Root cause
The model treated the two conditions as independent, without recognising that they are
not mutually exclusive.  In the PSCI spec, `CPU_DEFAULT_SUSPEND` validates the entry
point address **before** checking whether the CPU is already running.  The correct model
is a priority chain (error conditions take priority over success), not independent
implications.

### Correct form
```rust
// Correct (error checks take precedence):
(!AddrIsNonSecure(old_s, entry_point) && CpuIsOn(old_s, entry_point as Bits64)
    ==> result == PSCI_SUCCESS)
&& (AddrIsNonSecure(old_s, entry_point)
    ==> result == PSCI_INVALID_PARAMETERS)
```

### Proof witness (Z3-verified)
```rust
proof fn bug2_cpu_default_suspend_conflicting_implications(
    entry_point: Address,
    result: Int,
    s: S,
    old_s: S,
)
    requires
        CpuIsOn(old_s, entry_point as Bits64),
        AddrIsNonSecure(old_s, entry_point),
        psci_cpu_default_suspend_spec(entry_point, result, s, old_s),
    ensures
        false,  // INCONSISTENCY CONFIRMED BY Z3
{
    assert(result == PSCI_SUCCESS);              // from implication A: 0
    assert(result == PSCI_INVALID_PARAMETERS);   // from implication B: -2
    assert(PSCI_SUCCESS != PSCI_INVALID_PARAMETERS);
}
```

---

## Bug 3 — `psci_cpu_on_32_spec`: Truncation artifact replaces `||` with `&&`, creating dead error-code paths

### Severity
**High** — the spec silently drops coverage of five valid `CPU_ON` error codes
(`PSCI_DISABLED`, `PSCI_NOT_PRESENT`, `PSCI_INVALID_PARAMETERS`, `PSCI_INVALID_ADDRESS`,
`PSCI_INTERNAL_FAILURE`) in the precondition-violation branch.  Any implementation that
returns these codes when preconditions are not met cannot be verified against this spec.

### Location
`psci_generated_clean.rs`, function `psci_cpu_on_32_spec`, lines 308–311
Source: `psci_generated.rs`, line 424 (BSON-wrapped value field, truncated model output)

### Description
The third conjunct of `psci_cpu_on_32_spec` covers the case where the caller's
preconditions are **not** met.  Due to an output truncation in the model, `||` separators
between error-code disjuncts were dropped, leaving `&&`:

```rust
// Generated (BUGGY) — the negative-precondition branch:
!(preconditions) ==> (
    (result.is_Ok() && CpuIsOn(new_s, target_cpu))
    || (result.is_Err() && code == PSCI_ALREADY_ON)
    || (result.is_Err() && code == PSCI_ON_PENDING)
    || (result.is_Err() && code == PSCI_DISABLED)   // ← last || kept
        && code == PSCI_NOT_PRESENT                  // ← || became &&
        && code == PSCI_INVALID_PARAMETERS           // ← || became &&
        && code == PSCI_INVALID_ADDRESS              // ← || became &&
        && code == PSCI_INTERNAL_FAILURE             // ← || became &&
        && code == PSCI_DENIED                       // ← || became &&
)
```

Due to operator precedence (`&&` binds tighter than `||`), this parses as:

```
... || ( (result.is_Err() && code == PSCI_DISABLED)
         && code == PSCI_NOT_PRESENT
         && code == PSCI_INVALID_PARAMETERS
         && code == PSCI_INVALID_ADDRESS
         && code == PSCI_INTERNAL_FAILURE
         && code == PSCI_DENIED )
```

The last disjunct requires `code == PSCI_DISABLED (-8)` AND
`code == PSCI_NOT_PRESENT (-7)` simultaneously — impossible.  It is always false,
making those five error codes unreachable in the spec.

### Root cause
Model output was truncated mid-generation.  The cleanup post-processor preserved the
partial output, which had already emitted three correct `||` disjuncts before truncation
degraded subsequent separators to `&&`.

### Correct form
```rust
// Correct — all error codes as || disjuncts:
!(preconditions) ==> (
    (result.is_Ok() && CpuIsOn(new_s, target_cpu))
    || (result.is_Err() && code == PSCI_ALREADY_ON)
    || (result.is_Err() && code == PSCI_ON_PENDING)
    || (result.is_Err() && code == PSCI_DISABLED)
    || (result.is_Err() && code == PSCI_NOT_PRESENT)
    || (result.is_Err() && code == PSCI_INVALID_PARAMETERS)
    || (result.is_Err() && code == PSCI_INVALID_ADDRESS)
    || (result.is_Err() && code == PSCI_INTERNAL_FAILURE)
    || (result.is_Err() && code == PSCI_DENIED)
)
```

### Proof witness (Z3-verified)
```rust
// The dead disjunct is always false — arithmetic on PSCI constants:
proof fn bug3_cpu_on_dead_disjunct_always_false(code: PsciCode)
    requires
        code.as_int() == PSCI_DISABLED,    // -8
        code.as_int() == PSCI_NOT_PRESENT, // -7
    ensures
        false,  // -8 == -7 is impossible; CONFIRMED BY Z3
{
    assert(PSCI_DISABLED != PSCI_NOT_PRESENT);
}
```

---

## Bug 4 — `system_reset2_spec`: Vacuous postconditions — spec is a tautology

### Severity
**High** — the spec is always satisfied regardless of what `result` the implementation
returns.  It provides zero verification value: any implementation, correct or not, will
pass a check against this spec.

### Location
`psci_generated_clean.rs`, function `system_reset2_spec`, lines 455–462
Source: `psci_generated.rs`, lines 1320–1420 (model output)

### Description
Every postcondition in the generated spec takes the form:

```rust
!Condition(old_s) ==> result == new_result
```

where `new_result` is a free **input parameter** of the function.  Because `new_result`
is unconstrained, the caller can always choose `new_result = result` to satisfy every
such implication trivially.  The complete spec body:

```rust
// Generated (BUGGY):
(fid == PSCI_SYSTEM_RESET2_32_FID) ==> (
    (!CpuIsValid(old_s, entry_point)            ==> result == new_result)
    && (!AddrIsNonSecure(old_s, entry_point)    ==> result == new_result)
    && (!CpuIsOn(old_s, entry_point)            ==> result == new_result)
    && (!CpuIsOnPending(old_s, entry_point)     ==> result == new_result)
    && (!CallerIsLastCpu(old_s)                 ==> result == new_result)
    && (!TrustedOsMigrationInProgress(old_s)    ==> result == new_result)
    // ... [TRUNCATED]
)
```

All six conditions are independent of the actual result of `SYSTEM_RESET2`.

### Root cause
The model confused a local result variable with the function's own `result` parameter.
The intended semantics would compare `result` against specific PSCI return codes
(e.g., `PSCI_INVALID_PARAMETERS`, `PSCI_DENIED`) for each violated precondition, not
against an unconstrained `new_result` parameter.

### Correct form
```rust
// Correct — each violated precondition maps to a specific error code:
(fid == PSCI_SYSTEM_RESET2_32_FID) ==> (
    (!CpuIsValid(old_s, entry_point as Bits64)         ==> result.is_Err() && result.get_Err_0().as_int() == PSCI_INVALID_PARAMETERS)
    && (!AddrIsNonSecure(old_s, entry_point as Address) ==> result.is_Err() && result.get_Err_0().as_int() == PSCI_INVALID_ADDRESS)
    && (!CpuIsOn(old_s, entry_point as Bits64)          ==> result.is_Err() && result.get_Err_0().as_int() == PSCI_DENIED)
    // ...
)
```

### Proof witness (Z3-verified)
```rust
// Any result satisfies the spec when new_result is chosen freely:
proof fn bug4_system_reset2_tautology(
    s: S, fid: Bits64, entry_point: Address, reset_type: Bits64, cookie: Bits64,
    result: Result<(), PsciCode>, old_s: S, new_s: S,
)
    requires
        fid == PSCI_SYSTEM_RESET2_32_FID,
    ensures
        // For ANY result, spec holds with new_result := result
        exists|old_r: Result<(), PsciCode>, new_r: Result<(), PsciCode>|
            system_reset2_spec(s, fid, entry_point, reset_type, cookie,
                               result, old_r, new_r, old_s, new_s),
{
    assert(system_reset2_spec(
        s, fid, entry_point, reset_type, cookie, result, result, result, old_s, new_s,
    ));  // TAUTOLOGY CONFIRMED BY Z3
}
```

---

## Summary

| # | Spec | Bug type | Severity | Z3 verdict |
|---|------|----------|----------|------------|
| 1 | `affinity_info_spec` | Error codes conjuncted (`&&`) instead of disjuncted (`\|\|`) | Critical | `ensures false` proved |
| 2 | `psci_cpu_default_suspend_spec` | Non-exclusive antecedents with contradictory consequents | Critical | `ensures false` proved |
| 3 | `psci_cpu_on_32_spec` | Truncation converts `\|\|` to `&&` in error-code list, killing 5 error paths | High | Dead disjunct proved `false` |
| 4 | `system_reset2_spec` | All postconditions reference free parameter `new_result` — tautology | High | Universally satisfiable proved |

### Common patterns

- **Bugs 1 & 3**: The model generates correct disjunctive structure early in the output,
  then degrades to conjunctions as generation continues.  This is a known failure mode of
  autoregressive models that lose track of syntactic context over long outputs.

- **Bug 2**: The model generates independent implications for each precondition/result
  pair without encoding the priority ordering required when antecedents overlap.

- **Bug 4**: The model introduces extra parameters (here `new_result`) as a scaffold for
  expressing state transitions, then uses those parameters in postconditions instead of
  concrete values, producing vacuous constraints.

### Recommended mitigations

1. **Post-generation satisfiability check**: Run a lightweight Z3 pass after generation
   that tests `∃ inputs. spec(inputs) == true` for each spec function.  An unsatisfiable
   spec (Bugs 1, 2) is caught immediately.

2. **Tautology check**: Test `∀ inputs. spec(inputs) == true`.  A spec that is always
   true (Bug 4) is vacuous.

3. **Free-parameter detection**: Flag any spec parameter that appears only in a position
   `result == free_param` — this strongly suggests Bug 4.

4. **Truncation recovery**: When model output is truncated mid-disjunction, the cleanup
   post-processor should default to `|| true` rather than `&& ...` to avoid silently
   narrowing the spec (Bug 3).

---

## Verus-verified benchmark and two follow-up findings

The bugs above are now packaged as a machine-checked benchmark with a scorer — see
[`../BENCHMARK_VERUS_RMM.md`](../BENCHMARK_VERUS_RMM.md) and
[`../benchmark/verus_rmm/`](../benchmark/verus_rmm/). Two corrections/additions came
out of building it, both machine-checked in `benchmark/verus_rmm/new_findings.rs`:

- **Bug 5 (`RSI_ATTESTATION_TOKEN_CONTINUE`) is not alp14-specific** — the same
  witness produces the contradiction on eac5 and rel0 as well, so it has survived
  three releases unfixed.
- **`RMI_RTT_DESTROY` (alp14, borderline)** — the output summary table states
  "After execution: UNASSIGNED" unconditionally while the Success conditions give
  `UNASSIGNED_NS` for unprotected IPAs; contradiction verified.

Also recorded there: `inconsistency_analysis_rmm.py` asks whether a spec function is
unsatisfiable for *all* inputs, but these bugs are unsatisfiable only under a
*witness* — which is why its sweep of 89 alp14 gold specs reported 0 new
inconsistencies while missing a published bug.
