use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type PeId = u64;
pub type FfaInstance = u64;

pub struct PeStateT {
    pub regs: Seq<u64>,
}

pub struct S {
    pub pe_states: Map<PeId, PeStateT>,
}

pub const NOT_SUPPORTED: Int32 = -1;
pub const DENIED: Int32 = -6;

#[allow(non_upper_case_globals)]
pub const current_pe: PeId = 0;

pub open spec fn IsSecurePhysicalInstance(s: S, ffa_instance: FfaInstance) -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn NormalWorldWasPreempted(s: S, pe: PeId) -> bool;
pub open spec fn NormalWorldResumed(s: S, pe: PeId) -> bool;
pub open spec fn PeState(s: S, pe: PeId) -> PeStateT;
pub open spec fn SavedNormalWorldPeState(s: S, pe: PeId) -> PeStateT;
pub open spec fn CompletedByFfaFunctionInvocationViaEret(s: S, pe: PeId) -> bool;

} // verus!
