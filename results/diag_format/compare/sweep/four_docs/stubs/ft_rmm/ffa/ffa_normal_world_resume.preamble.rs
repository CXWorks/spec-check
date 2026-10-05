use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type SecurePhysicalInstance = u64;
pub type CurrentPe = u64;
pub type PeState = u64;
pub type FfaFunctionId = u32;

pub struct S {
    pub dummy: u64,
}

pub spec const DENIED: Int32 = (-6) as i32;
pub spec const NOT_SUPPORTED: Int32 = (-1) as i32;
pub spec const ERET: FfaFunctionId = 0x8400008A;

pub open spec fn IsSecurePhysicalInstance(s: S, instance: SecurePhysicalInstance) -> bool;
pub open spec fn NormalWorldWasPreempted(s: S, current_pe: CurrentPe) -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn NormalWorldExecutionResumed(s: S, current_pe: CurrentPe) -> bool;
pub open spec fn NormalWorldPeState(s: S, current_pe: CurrentPe) -> PeState;
pub open spec fn SavedPreemptedPeState(s: S, current_pe: CurrentPe) -> PeState;
pub open spec fn CompletedByFfaFunctionInvocation(s: S, func: FfaFunctionId) -> bool;

} // verus!
