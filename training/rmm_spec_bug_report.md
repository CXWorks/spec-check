# ARM CCA RMM Specification Inconsistency Report

Proofs in `rmm_bugs.rs`. Each `proof fn` carries `ensures false` and is accepted by Verus,
confirming the contradiction is machine-checked. All three potential bugs were found by
inspecting the gold Verus annotations (`specs/alp14/`) against the RMM spec section text
(`sections/alp14/`). Bug 6 is a **false positive** — the ARM spec is consistent there;
the inconsistency lives in the gold annotation.

Spec document: **ARM CCA Realm Management Monitor (RMM) Specification** (functional ID:
  RMM Architecture Specification, version alp14).

---

## Bug 4 — RMI_PDEV_STOP: incomplete failure-condition ordering

**Spec section:** §B4.3.20.2 and §B4.3.20.2.1
**Gold annotation:** `specs/alp14/rmi_pdev_stop_spec.rs`
**Proof fn:** `bug4_rmi_pdev_stop_dual_error`

### Conflicting sources

| Failure condition ID | Pre-condition | Post-condition |
|----------------------|--------------|----------------|
| `pdev_align` (§B4.3.20.2) | `!AddrIsGranuleAligned(pdev_ptr)` | `RMI_ERROR_INPUT` |
| `pdev_state` (§B4.3.20.2) | `pdev.state ∈ {COMMUNICATING, STOPPING, STOPPED}` | `RMI_ERROR_DEVICE` |

The failure condition ordering in §B4.3.20.2.1 states only:

```
[da_supp] < [pdev_align, pdev_bound, pdev_gran_state]
[pdev_gran_state] < [pdev_state, num_vdevs]
```

There is **no ordering edge** between `pdev_align` and `pdev_state`. The ordering
transitively relates `pdev_gran_state → pdev_state`, but leaves `pdev_align` and
`pdev_state` incomparable.

### Witness state

```
ImplFeatures().feat_da == FEATURE_TRUE        (da_supp does not fire)
!AddrIsGranuleAligned(pdev_ptr)               (pdev_align fires → RMI_ERROR_INPUT)
GranuleAt(pdev_ptr).state == PDEV             (pdev_gran_state does not fire)
pdev.state == PDEV_STOPPING                   (pdev_state fires → RMI_ERROR_DEVICE)
```

- `pdev_align` requires `result == RMI_ERROR_INPUT` (== 1)
- `pdev_state` requires `result == RMI_ERROR_DEVICE` (== 5)
- No ordering between them → both required simultaneously → `1 == 5`, contradiction.

### Verus proof

```rust
proof fn bug4_rmi_pdev_stop_dual_error(pdev_ptr: int, result: int, old_s: int)
    requires
        ImplFeatsDa(old_s) == FEATURE_TRUE,
        !AddrIsGranuleAligned(old_s, pdev_ptr),
        PdevState(old_s, pdev_ptr) == PDEV_STOPPING,
        !AddrIsGranuleAligned(old_s, pdev_ptr) ==> result == RMI_ERROR_INPUT,
        (PdevState(old_s, pdev_ptr) == PDEV_COMMUNICATING
         || PdevState(old_s, pdev_ptr) == PDEV_STOPPING
         || PdevState(old_s, pdev_ptr) == PDEV_STOPPED) ==> result == RMI_ERROR_DEVICE,
    ensures false
{}
```

### Note

The visual DAG diagram in §B4.3.20.2.1 groups `pdev_align` and `pdev_gran_state` on the
same tier above `pdev_state`, suggesting an implicit tier ordering. However the textual
ordering declaration is missing the edges
`[pdev_align, pdev_bound] < [pdev_state, num_vdevs]`.
Any formal reading of the stated ordering (as used in our Verus annotation) produces the
contradiction.

---

## Bug 5 — RSI_ATTESTATION_TOKEN_CONTINUE: explicit "no ordering" with conflicting conditions

**Spec section:** §B5.3.1.2 and §B5.3.1.2.1
**Gold annotation:** `specs/alp14/rsi_attestation_token_continue_spec.rs`
**Proof fn:** `bug5_rsi_attestation_token_continue_dual_error`

### Conflicting sources

| Failure condition ID | Pre-condition | Post-condition |
|----------------------|--------------|----------------|
| `addr_align` (§B5.3.1.2) | `!AddrIsGranuleAligned(addr)` | `result == RSI_ERROR_INPUT` |
| `state` (§B5.3.1.2) | `rec.attest_state != ATTEST_IN_PROGRESS` | `result == RSI_ERROR_STATE` |

The ordering section §B5.3.1.2.1 explicitly states:

> "The RSI_ATTESTATION_TOKEN_CONTINUE command does not have any failure condition orderings."

With no ordering, all failure conditions have equal priority. When both `addr_align` and
`state` trigger simultaneously, the spec simultaneously requires `RSI_ERROR_INPUT` and
`RSI_ERROR_STATE`.

### Witness state

```
!AddrIsGranuleAligned(addr)                    (addr_align fires → RSI_ERROR_INPUT)
rec.attest_state != ATTEST_IN_PROGRESS         (state fires → RSI_ERROR_STATE)
```

- `addr_align` requires `result == RSI_ERROR_INPUT` (== 1)
- `state` requires `result == RSI_ERROR_STATE` (== 2)
- No ordering → both required simultaneously → `1 == 2`, contradiction.

### Verus proof

```rust
proof fn bug5_rsi_attestation_token_continue_dual_error(addr: int, result: int, old_s: int)
    requires
        !AddrIsGranuleAligned(old_s, addr),
        CurrentRecAttestState(old_s) != ATTEST_IN_PROGRESS,
        !AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT,
        CurrentRecAttestState(old_s) != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE,
    ensures false
{}
```

### Suggested fix

Add a failure condition ordering:

```
[addr_align, addr_bound, addr_empty, offset_bound, size_overflow, size_bound] < [state]
```

This ensures address/range checks take priority over the attestation-state check,
consistent with the pattern seen in other RSI commands.

---

## Bug 6 — RSI_VDEV_VALIDATE_MAPPING — FALSE POSITIVE (annotation error, not spec error)

**Spec section:** §B5.3.19.2 and §B5.3.19.2.1
**Gold annotation:** `specs/alp14/rsi_vdev_validate_mapping_spec.rs`
**Proof fn:** `bug6_rsi_vdev_validate_mapping_dual_error` (proves annotation inconsistent)

### Analysis

The ARM spec §B5.3.19.2.1 explicitly states the ordering:

```
[da_en] < [vdev_id]
```

meaning: if `da_en` (→ `RSI_ERROR_STATE`) fires, it takes priority over
`vdev_id` (→ `RSI_ERROR_INPUT`). The spec IS consistent.

The gold Verus annotation in `rsi_vdev_validate_mapping_spec.rs` encodes both
conditions as unconditional implications without capturing this priority:

```rust
(CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
&& (VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id) ==> result == RSI_ERROR_INPUT)
```

When both preconditions hold, the annotation forces
`result == RSI_ERROR_STATE ∧ result == RSI_ERROR_INPUT` — a contradiction that Verus
can prove, but this contradiction is an **annotation defect**, not a spec defect.

### Correct annotation

The `vdev_id` implication should be guarded by `da_en` not firing:

```rust
(VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id)
 && CurrentRealm(old_s).feat_da == FEATURE_TRUE ==> result == RSI_ERROR_INPUT)
```

---

## Coverage summary

| Command | Section | Bug type | Status |
|---------|---------|----------|--------|
| RMI_PDEV_STOP | §B4.3.20.2.1 | Incomplete failure condition ordering | **Confirmed (machine-checked)** |
| RSI_ATTESTATION_TOKEN_CONTINUE | §B5.3.1.2.1 | Explicit "no ordering" + conflicting conditions | **Confirmed (machine-checked)** |
| RSI_VDEV_VALIDATE_MAPPING | §B5.3.19.2.1 | Annotation encodes ordering incorrectly | **False positive** (annotation bug) |

| Spec section | Commands checked | Spec inconsistencies proved | Annotation bugs found |
|-------------|-----------------|----------------------------|-----------------------|
| RMM alp14 (test set, commands) | 11 | **2** (§B4.3.20.2.1, §B5.3.1.2.1) | **1** (§B5.3.19.2.1) |
