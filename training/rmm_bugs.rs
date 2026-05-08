use vstd::prelude::*;

verus! {

// ── Encode RsiCommandReturnCode as integers ──────────────────────────────────
pub spec const RSI_SUCCESS:       int = 0;
pub spec const RSI_ERROR_INPUT:   int = 1;
pub spec const RSI_ERROR_STATE:   int = 2;
pub spec const RSI_ERROR_DEVICE:  int = 5;

// ── Encode RmiStatusCode as integers ────────────────────────────────────────
pub spec const RMI_ERROR_INPUT:        int = 1;
pub spec const RMI_ERROR_NOT_SUPPORTED:int = 4;
pub spec const RMI_ERROR_DEVICE:       int = 5;

// ── Opaque state ─────────────────────────────────────────────────────────────
pub open spec fn ImplFeatsDa(s: int) -> int;
pub open spec fn AddrIsGranuleAligned(s: int, addr: int) -> bool;
pub open spec fn PaIsDelegable(s: int, addr: int) -> bool;
pub open spec fn GranuleState(s: int, addr: int) -> int;
pub open spec fn PdevState(s: int, addr: int) -> int;
pub open spec fn PdevNumVdevs(s: int, addr: int) -> int;
pub open spec fn CurrentRecAttestState(s: int) -> int;
pub open spec fn CurrentRealmFeatDa(s: int) -> int;
pub open spec fn VdevIdIsFree(s: int, vdev_id: int) -> bool;

pub spec const FEATURE_TRUE:      int = 1;
pub spec const ATTEST_IN_PROGRESS:int = 1;
pub spec const PDEV_COMMUNICATING:int = 2;
pub spec const PDEV_STOPPING:     int = 3;
pub spec const PDEV_STOPPED:      int = 4;


// ============================================================
// Bug 4 — rmi_pdev_stop: dual error code contradiction
//
// Source A — spec line 3 (address check):
//   !AddrIsGranuleAligned ==> result == RMI_ERROR_INPUT
//
// Source B — spec line 7 (device state check):
//   state ∈ {PDEV_COMMUNICATING, PDEV_STOPPING, PDEV_STOPPED}
//       ==> result == RMI_ERROR_DEVICE
//
// Witness: !AddrIsGranuleAligned ∧ state == PDEV_STOPPING
//   Source A → result == RMI_ERROR_INPUT (== 1)
//   Source B → result == RMI_ERROR_DEVICE (== 5)
//   Contradiction since 1 ≠ 5.
// ============================================================

proof fn bug4_rmi_pdev_stop_dual_error(
    pdev_ptr: int,
    result: int,
    old_s: int,
)
    requires
        ImplFeatsDa(old_s) == FEATURE_TRUE,           // not NOT_SUPPORTED
        !AddrIsGranuleAligned(old_s, pdev_ptr),        // → RMI_ERROR_INPUT
        PdevState(old_s, pdev_ptr) == PDEV_STOPPING,   // → RMI_ERROR_DEVICE
        // Spec implications:
        !AddrIsGranuleAligned(old_s, pdev_ptr) ==> result == RMI_ERROR_INPUT,
        (PdevState(old_s, pdev_ptr) == PDEV_COMMUNICATING
         || PdevState(old_s, pdev_ptr) == PDEV_STOPPING
         || PdevState(old_s, pdev_ptr) == PDEV_STOPPED) ==> result == RMI_ERROR_DEVICE,
    ensures false
{}


// ============================================================
// Bug 5 — rsi_attestation_token_continue: dual error code
//
// Source A — address check:
//   !AddrIsGranuleAligned ==> result == RSI_ERROR_INPUT
//
// Source B — attest state check:
//   attest_state != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE
//
// Witness: !AddrIsGranuleAligned ∧ attest_state != ATTEST_IN_PROGRESS
//   Source A → result == RSI_ERROR_INPUT (== 1)
//   Source B → result == RSI_ERROR_STATE (== 2)
//   Contradiction since 1 ≠ 2.
// ============================================================

proof fn bug5_rsi_attestation_token_continue_dual_error(
    addr: int,
    result: int,
    old_s: int,
)
    requires
        !AddrIsGranuleAligned(old_s, addr),
        CurrentRecAttestState(old_s) != ATTEST_IN_PROGRESS,
        !AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT,
        CurrentRecAttestState(old_s) != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE,
    ensures false
{}


// ============================================================
// Bug 6 — rsi_vdev_validate_mapping: dual error code  [FALSE POSITIVE]
//
// NOTE: §B5.3.19.2.1 explicitly states [da_en] < [vdev_id], so the ARM
// spec is consistent: da_en takes priority and returns RSI_ERROR_STATE.
// The contradiction below is in the gold Verus annotation
// (rsi_vdev_validate_mapping_spec.rs) which omits the priority guard,
// encoding the vdev_id implication unconditionally instead of guarding
// it with `feat_da == FEATURE_TRUE`. This is an annotation defect.
//
// Source A — feat_da check (§B5.3.19.2, da_en):
//   feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE
//
// Source B — vdev_id check (§B5.3.19.2, vdev_id):
//   VdevIdIsFree(...) ==> result == RSI_ERROR_INPUT   ← missing priority guard
//
// Witness: feat_da != FEATURE_TRUE ∧ VdevIdIsFree
//   Source A → result == RSI_ERROR_STATE (== 2)
//   Source B → result == RSI_ERROR_INPUT (== 1)
//   Contradiction since 2 ≠ 1.  (Spec says da_en wins; annotation doesn't.)
// ============================================================

proof fn bug6_rsi_vdev_validate_mapping_dual_error(
    vdev_id: int,
    result: int,
    old_s: int,
)
    requires
        CurrentRealmFeatDa(old_s) != FEATURE_TRUE,
        VdevIdIsFree(old_s, vdev_id),
        CurrentRealmFeatDa(old_s) != FEATURE_TRUE ==> result == RSI_ERROR_STATE,
        VdevIdIsFree(old_s, vdev_id) ==> result == RSI_ERROR_INPUT,
    ensures false
{}

} // verus!
