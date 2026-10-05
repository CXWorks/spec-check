use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub enum FfaInstance {
    SecurePhysical,
    NonSecurePhysical,
    SecureVirtual,
    NonSecureVirtual,
}

pub const FF_A_SECURE_PHYSICAL: FfaInstance = FfaInstance::SecurePhysical;

pub const FFA_ERROR_DENIED: int32 = -6;
pub const FFA_ERROR_NOT_SUPPORTED: int32 = -1;

pub struct S {
    pub ffa_instance: FfaInstance,
    pub normal_world_preempted: bool,
}

pub open spec fn ResultEqual(result: int32, expected: int32) -> bool;

} // verus!
