use vstd::prelude::*;
verus! {

pub type FfaReturnCode = i32;

pub const NOT_SUPPORTED: FfaReturnCode = -1;
pub const DENIED: FfaReturnCode = -6;

pub struct PeState {
    pub regs: Seq<u64>,
}

pub struct S {
    pub secure_physical_instance: bool,
    pub normal_world_preempted: bool,
    pub normal_world_resumed: bool,
    pub normal_world_pe_state: PeState,
    pub saved_normal_world_pe_state: PeState,
}

pub open spec fn IsSecurePhysicalFfaInstance(s: S) -> bool;

pub open spec fn NormalWorldPreempted(s: S) -> bool;

pub open spec fn IsFfaError(r: FfaReturnCode) -> bool;

pub open spec fn NormalWorldResumed(s: S) -> bool;

pub open spec fn NormalWorldPeState(s: S) -> PeState;

pub open spec fn SavedNormalWorldPeState(s: S) -> PeState;

} // verus!
