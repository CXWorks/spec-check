use vstd::prelude::*;

verus! {

// ============================================================
// Spec-faithful inconsistency proofs for ARM firmware specs
//
// Each proof fn encodes two conflicting constraints from the
// same spec section and shows they imply false.
// ============================================================

// ---------------------------------------------------------------------------
// Common integer types
// ---------------------------------------------------------------------------

type Int64 = int;
type Int32 = int;

// ---------------------------------------------------------------------------
// SDEI constants (DEN0054C)
// ---------------------------------------------------------------------------

pub spec const SDEI_SUCCESS:            int = 0;
pub spec const SDEI_NOT_SUPPORTED:      int = -1;
pub spec const SDEI_INVALID_PARAMETERS: int = -2;
pub spec const SDEI_DENIED:             int = -3;

// ---------------------------------------------------------------------------
// DRTM constants (DEN0113 v1.4)
// ---------------------------------------------------------------------------

pub spec const DRTM_SUCCESS:            int = 0;
pub spec const DRTM_NOT_SUPPORTED:      int = -1;
pub spec const DRTM_DENIED:             int = -3;


// ============================================================
// BUG 1 — SDEI_SHARED_RESET (DEN0054C §5.1.19)
//
// Source A — return-value table (§5.1.19 Interface):
//   DENIED ⟺ "Event was running while this call was invoked."
//
// Source B — usage/client-responsibilities text (§5.1.19.1):
//   "The call will return a DENIED error if there was at least
//    one shared event that was running OR at least one
//    interrupt-event binding (private or shared) that was still
//    registered."
//
// Witness: SDEI supported, no shared event running, but a
//          bound interrupt still exists.
//   Table  → result ≠ DENIED  (no running event)
//   Text   → result == DENIED (binding exists)
// ============================================================

pub open spec fn sdei_supported() -> bool;
pub open spec fn some_shared_event_running() -> bool;
pub open spec fn some_interrupt_binding_exists() -> bool;

pub open spec fn sdei_shared_reset_table(result: Int64) -> bool {
    (!sdei_supported() ==> result == SDEI_NOT_SUPPORTED)
    && (sdei_supported() ==>
        (result == SDEI_DENIED <==> some_shared_event_running()))
}

pub open spec fn sdei_shared_reset_text(result: Int64) -> bool {
    (!sdei_supported() ==> result == SDEI_NOT_SUPPORTED)
    && (sdei_supported() ==>
        (result == SDEI_DENIED <==>
            (some_shared_event_running() || some_interrupt_binding_exists())))
}

proof fn bug1_sdei_shared_reset(result: Int64)
    requires
        sdei_supported(),
        !some_shared_event_running(),
        some_interrupt_binding_exists(),
        sdei_shared_reset_table(result),
        sdei_shared_reset_text(result),
    ensures false
{}


// ============================================================
// BUG 2 — SDEI_INTERRUPT_BIND (DEN0054C §5.1.14)
//
// Source A — description / return table (§5.1.14):
//   "Binding any type of interrupt that is already bound will
//    return the same event number."  (event number > 0 = success)
//
// Source B — client responsibilities (§5.1.14.3):
//   "DENIED is returned if the interrupt is not in Inactive
//    state."
//
// Witness: interrupt is already bound (and therefore not
//          Inactive — it is dispatcher-managed).
//   Table → result > 0  (returns event number)
//   Text  → result == DENIED  (== −3)
// ============================================================

pub open spec fn interrupt_already_bound(intr: u32) -> bool;
pub open spec fn interrupt_is_inactive(intr: u32) -> bool;

pub open spec fn sdei_interrupt_bind_table(intr: u32, result: Int64) -> bool {
    interrupt_already_bound(intr) ==> result > 0
}

pub open spec fn sdei_interrupt_bind_client(intr: u32, result: Int64) -> bool {
    !interrupt_is_inactive(intr) ==> result == SDEI_DENIED
}

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


// ============================================================
// BUG 3 — DRTM_ENABLE_SECURE_INTERRUPTS (DEN0113 v1.4 §3.11)
//
// Source A — return-value table (§3.11 Interface):
//   DENIED ⟺ "A dynamic launch has not occurred, OR
//              Secure interrupts are not disabled."
//   i.e., ¬ (launch_occurred ∧ sec_interrupts_disabled)
//
// Source B — implementation responsibilities (§3.11.3):
//   "If the disabling of Secure interrupts was not requested
//    in DRTM_PARAMETERS, the implementation MUST return DENIED."
//   i.e., ¬ requested_in_params → result == DENIED
//
// Witness: hardware-backed implementation where a dynamic
//          launch HAS occurred and Secure interrupts ARE
//          disabled (§3.11.1: "Hardware-backed implementation:
//          Secure interrupts are always disabled during a
//          dynamic launch"), but the caller did NOT request
//          disabling in DRTM_PARAMETERS.
//
//   Table → NOT DENIED  (launch occurred ∧ interrupts disabled)
//   Impl  → MUST return DENIED  (not requested in params)
// ============================================================

pub open spec fn drtm_launch_occurred() -> bool;
pub open spec fn sec_interrupts_disabled() -> bool;
pub open spec fn disable_requested_in_params() -> bool;

pub open spec fn drtm_enable_sec_intr_table(result: Int64) -> bool {
    result == DRTM_DENIED <==>
        !(drtm_launch_occurred() && sec_interrupts_disabled())
}

pub open spec fn drtm_enable_sec_intr_impl(result: Int64) -> bool {
    !disable_requested_in_params() ==> result == DRTM_DENIED
}

proof fn bug3_drtm_enable_secure_interrupts(result: Int64)
    requires
        drtm_launch_occurred(),
        sec_interrupts_disabled(),       // always true in hardware-backed impl
        !disable_requested_in_params(),  // caller did not request it
        drtm_enable_sec_intr_table(result),
        drtm_enable_sec_intr_impl(result),
    ensures false
{}


} // verus!
