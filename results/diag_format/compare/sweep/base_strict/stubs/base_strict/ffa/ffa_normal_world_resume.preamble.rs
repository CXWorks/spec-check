use vstd::prelude::*;
verus! {

pub type Int32 = i32;

pub struct FfaInstance {
    pub id: nat,
}

pub struct Pe {
    pub id: nat,
}

pub struct PeStateValue {
    pub value: nat,
}

pub struct S {
    pub instance: FfaInstance,
    pub pe: Pe,
}

pub const NOT_SUPPORTED: Int32 = -1;
pub const DENIED: Int32 = -6;

pub open spec fn ffa_instance(s: S) -> FfaInstance;

pub open spec fn current_pe(s: S) -> Pe;

pub open spec fn IsSecurePhysicalInstance(inst: FfaInstance) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn NormalWorldWasPreempted(pe: Pe) -> bool;

pub open spec fn NormalWorldResumed(pe: Pe) -> bool;

pub open spec fn PeState(pe: Pe) -> PeStateValue;

pub open spec fn SavedNormalWorldPeState(pe: Pe) -> PeStateValue;

pub open spec fn CompletedByFfaFunctionInvocationViaEret(pe: Pe) -> bool;

} // verus!
