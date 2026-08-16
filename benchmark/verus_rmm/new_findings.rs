use vstd::prelude::*;
verus! {

// RMI_RTT_DESTROY (alp14 §B4.3.39): do the summary table and the Success
// conditions table agree for an UNPROTECTED IPA?
//
// Source A -- output summary table (§B4.3.39.1.3):
//   "Target RTT exists and is not live -> RMI_SUCCESS ...
//    walk.rtte.state After execution: UNASSIGNED and RIPAS is DESTROYED"
//   Stated unconditionally; the table never mentions protection.
//
// Source B -- Success conditions (§B4.3.39.3):
//   state_prot    pre AddrIsProtected  -> post state == UNASSIGNED
//   state_unprot  pre !AddrIsProtected -> post state == UNASSIGNED_NS
//
// Witness: command succeeds on an unprotected IPA.

pub enum RtteState { UNASSIGNED, UNASSIGNED_NS, TABLE, ASSIGNED }

pub open spec fn addr_is_protected() -> bool;
pub open spec fn succeeded() -> bool;

// Source A: summary table, unguarded
pub open spec fn summary_table(new_state: RtteState) -> bool {
    succeeded() ==> new_state == RtteState::UNASSIGNED
}

// Source B: success conditions, guarded by protection
pub open spec fn success_conditions(new_state: RtteState) -> bool {
    (succeeded() && addr_is_protected()  ==> new_state == RtteState::UNASSIGNED)
    && (succeeded() && !addr_is_protected() ==> new_state == RtteState::UNASSIGNED_NS)
}

pub proof fn rtt_destroy_summary_vs_conditions(new_state: RtteState)
    requires
        succeeded(),
        !addr_is_protected(),
        summary_table(new_state),
        success_conditions(new_state),
    ensures false
{}

} // verus!

// ============================================================
// FINDING 2 — RSI_ATTESTATION_TOKEN_CONTINUE dual error is NOT alp14-specific
//
// training/rmm_bugs.rs records bug 5 as an alp14 finding. Re-running its witness
// against the eac5 and rel0 gold annotations shows the contradiction holds in
// every released version checked: ARM has not fixed it across three releases.
//
//   Source A -- address check:  !AddrIsGranuleAligned(addr) ==> RSI_ERROR_INPUT
//   Source B -- attest state:   attest_state != ATTEST_IN_PROGRESS ==> RSI_ERROR_STATE
//   Witness:  both preconditions hold simultaneously; the spec states no ordering
//             between them, so `result` must equal two distinct codes at once.
//
// Verified in this session against training-dataset/specs/{eac5,rel0,alp14}/
// rsi_attestation_token_continue_spec.rs -- `ensures false` accepted for all three.
// Reproduce:  python3 benchmark/verus_rmm/run_bench.py --version eac5 \
//                 --gen-dir training-dataset/specs/eac5 --gen-pattern '{cmd}_spec.rs'
//             (item eac5:RSI_ATTESTATION_TOKEN_CONTINUE:dual_error)
// ============================================================
