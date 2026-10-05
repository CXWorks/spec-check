use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub type FfaFunctionId = u32;

pub struct Instance {
    pub id: nat,
}

pub struct PeState {
    pub regs: Seq<u64>,
}

pub struct Pe {
    pub id: nat,
}

pub struct S {
    pub instance: Instance,
    pub current_pe: Pe,
}

pub const DENIED: Int32 = -6;

pub const NOT_SUPPORTED: Int32 = -1;

pub const ERET: FfaFunctionId = 0x8400_0096;

pub open spec fn IsSecurePhysicalInstance(instance: Instance) -> bool;

pub open spec fn NormalWorldWasPreempted(pe: Pe) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn NormalWorldExecutionResumed(pe: Pe) -> bool;

pub open spec fn NormalWorldPeState(pe: Pe) -> PeState;

pub open spec fn SavedPreemptedPeState(pe: Pe) -> PeState;

pub open spec fn CompletedByFfaFunctionInvocation(s: S, func: FfaFunctionId) -> bool;

} // verus!
