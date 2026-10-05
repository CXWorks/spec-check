use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;
pub type ExceptionLevel = u32;
pub type SecurityState = u32;
pub type FfaInstance = u32;

pub struct SCR_EL3_Reg {
    pub FIQ: u64,
}

pub struct S {
    pub current_instance: FfaInstance,
    pub SCR_EL3: SCR_EL3_Reg,
}

impl S {
    pub open spec fn CallerEl(self) -> ExceptionLevel;
    pub open spec fn SecurityStateOnReturn(self) -> SecurityState;
    pub open spec fn ReturnEl(self) -> ExceptionLevel;
}

pub const FFA_ERROR: UInt32 = 0x84000060;
pub const FFA_SUCCESS: UInt32 = 0x84000061;

pub const NOT_SUPPORTED: Int32 = -1;

pub const S_EL1: ExceptionLevel = 1;
pub const S_EL2: ExceptionLevel = 2;

pub const Secure: SecurityState = 0;

pub open spec fn IsSupportedFfaInstance(instance: FfaInstance) -> bool;
pub open spec fn ResultEqual(a: UInt32, b: UInt32) -> bool;
pub open spec fn PendingInterruptHandledByEl3Firmware() -> bool;

} // verus!
