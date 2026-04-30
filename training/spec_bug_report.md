# ARM Firmware Spec Inconsistency Report

Proofs in `spec_bugs.rs`. Each `proof fn` carries `ensures false` and is accepted by Verus, confirming the contradiction is machine-checked.

SCMI (DEN0056F v4.0) and FF-A (DEN0077A v1.3): no logical contradiction found within the section text available. Potential ambiguities exist (e.g., FFA_NOTIFICATION_BITMAP_DESTROY DENIED condition, SCMI POWER_STATE_SET async/sync semantics for AP domains) but do not rise to provable inconsistency with available predicates.

---

## Bug 1 — SDEI_SHARED_RESET: contradictory DENIED condition

**Spec:** DEN0054C §5.1.19
**Proof fn:** `bug1_sdei_shared_reset`

### Conflicting sources

| Source | DENIED condition |
|--------|-----------------|
| Return-value table (§5.1.19 Interface) | "Event was running while this call was invoked" |
| Usage + Client responsibilities text (§5.1.19.1) | "at least one shared event that was running **OR** at least one interrupt-event binding (private or shared) that was still registered" |

### Witness state

SDEI supported, no shared event running (`!some_shared_event_running()`), but a bound interrupt still exists (`some_interrupt_binding_exists()`).

- Table encodes: `result == SDEI_DENIED ⟺ some_shared_event_running()` → result ≠ DENIED
- Text encodes: `result == SDEI_DENIED ⟺ some_shared_event_running() ∨ some_interrupt_binding_exists()` → result == DENIED

Together they force `result == SDEI_DENIED ∧ result ≠ SDEI_DENIED`.

### Verus proof

```rust
proof fn bug1_sdei_shared_reset(result: Int64)
    requires
        sdei_supported(),
        !some_shared_event_running(),
        some_interrupt_binding_exists(),
        sdei_shared_reset_table(result),
        sdei_shared_reset_text(result),
    ensures false
{}
```

---

## Bug 2 — SDEI_INTERRUPT_BIND: already-bound interrupt treatment

**Spec:** DEN0054C §5.1.14
**Proof fn:** `bug2_sdei_interrupt_bind`

### Conflicting sources

| Source | Rule |
|--------|------|
| Description / return table (§5.1.14) | "Binding any type of interrupt that is already bound will return the same event number" (positive integer = success) |
| Client responsibilities (§5.1.14.3) | "DENIED is returned if the interrupt is not in Inactive state" |

### Witness state

Interrupt is already bound (`interrupt_already_bound(intr)`), therefore not in Inactive state — once promoted to an SDEI event the interrupt is dispatcher-managed, not idle.

- Table encodes: `interrupt_already_bound(intr) → result > 0`
- Client text encodes: `!interrupt_is_inactive(intr) → result == SDEI_DENIED` (== −3)

Together: `result > 0 ∧ result == −3`.

### Verus proof

```rust
proof fn bug2_sdei_interrupt_bind(intr: u32, result: Int64)
    requires
        sdei_supported(),
        interrupt_already_bound(intr),
        !interrupt_is_inactive(intr),
        sdei_interrupt_bind_table(intr, result),
        sdei_interrupt_bind_client(intr, result),
    ensures false
{
    assert(result > 0);
    assert(result == SDEI_DENIED);
    assert(SDEI_DENIED == -3int);
}
```

---

## Bug 3 — DRTM_ENABLE_SECURE_INTERRUPTS: hardware-backed DENIED condition

**Spec:** DEN0113 v1.4 §3.11
**Proof fn:** `bug3_drtm_enable_secure_interrupts`

### Conflicting sources

| Source | DENIED condition |
|--------|-----------------|
| Return-value table (§3.11 Interface) | "A dynamic launch has not occurred, OR Secure interrupts are not disabled" i.e., `¬(launch_occurred ∧ sec_interrupts_disabled)` |
| Implementation responsibilities (§3.11.3) | "If the disabling of Secure interrupts was not requested in DRTM_PARAMETERS, the implementation **MUST** return DENIED" i.e., `¬requested_in_params → DENIED` |

### Witness state

Hardware-backed implementation (§3.11.1: "Hardware-backed implementation: Secure interrupts are **always** disabled during a dynamic launch") where:
- A dynamic launch has occurred (`drtm_launch_occurred()`)
- Secure interrupts are disabled (`sec_interrupts_disabled()`) — guaranteed by hardware regardless of params
- Disabling was not explicitly requested (`!disable_requested_in_params()`)

- Table: `result == DRTM_DENIED ⟺ ¬(true ∧ true)` → result ≠ DENIED → should return SUCCESS
- Impl: `¬false → result == DRTM_DENIED` → must return DENIED

Together: `result == DRTM_DENIED ∧ result ≠ DRTM_DENIED`.

### Verus proof

```rust
proof fn bug3_drtm_enable_secure_interrupts(result: Int64)
    requires
        drtm_launch_occurred(),
        sec_interrupts_disabled(),
        !disable_requested_in_params(),
        drtm_enable_sec_intr_table(result),
        drtm_enable_sec_intr_impl(result),
    ensures false
{}
```

---

## Coverage summary

| Spec | Commands checked | Inconsistencies proved |
|------|-----------------|----------------------|
| SDEI (DEN0054C) | 19 | **2** (§5.1.19, §5.1.14) |
| DRTM (DEN0113) | 10 | **1** (§3.11) |
| SCMI (DEN0056F) | 22 | 0 |
| FF-A (DEN0077A) | 37 | 0 |
