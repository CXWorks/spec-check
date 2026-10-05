use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum ConduitType {
    Smc,
    Hvc,
}

pub enum ExceptionLevel {
    EL0,
    EL1,
    EL2,
    EL3,
    S_EL0,
    S_EL1,
    S_EL2,
}

pub struct S {
    pub conduit: ConduitType,
    pub caller_el: ExceptionLevel,
}

pub const S_EL1: ExceptionLevel = ExceptionLevel::S_EL1;
pub const S_EL2: ExceptionLevel = ExceptionLevel::S_EL2;

pub const FFA_SUCCESS: UInt32 = 0;
pub const FFA_ERROR_NOT_SUPPORTED: UInt32 = 0xFFFF_FFFF;

pub open spec fn IsSupportedFfaInstance(ffa_instance: UInt32, conduit: ConduitType) -> bool;
pub open spec fn ResultEqual(result: UInt32, expected: UInt32) -> bool;
pub open spec fn PendingGroup0InterruptHandledByEl3() -> bool;
pub open spec fn El3SwitchedSecurityState() -> bool;
pub open spec fn ReturnsToCallingElInSecureState() -> bool;

} // verus!
