use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type CurrentInstance = u32;
pub type CallerEl = u8;
pub type ElLevel = u8;
pub type SecurityState = u8;

pub struct S {
    pub dummy: int,
}

#[allow(non_snake_case)]
pub struct ScrEl3Reg {
    pub FIQ: u32,
}

pub const SCR_EL3: ScrEl3Reg = ScrEl3Reg { FIQ: 0 };

pub const FFA_ERROR: UInt32 = 0x84000060;
pub const FFA_SUCCESS: UInt32 = 0x84000061;

pub const NOT_SUPPORTED: Int32 = -1;

pub const S_EL1: ElLevel = 1;
pub const S_EL2: ElLevel = 2;

#[allow(non_upper_case_globals)]
pub const Secure: SecurityState = 0;

#[allow(non_snake_case)]
pub open spec fn IsSupportedFfaInstance(s: S, current_instance: CurrentInstance) -> bool;

#[allow(non_snake_case)]
pub open spec fn ResultEqual(a: UInt32, b: UInt32) -> bool;

#[allow(non_snake_case)]
pub open spec fn CallerEl(s: S) -> ElLevel;

#[allow(non_snake_case)]
pub open spec fn ReturnEl(s: S) -> ElLevel;

#[allow(non_snake_case)]
pub open spec fn PendingInterruptHandledByEl3Firmware(s: S) -> bool;

#[allow(non_snake_case)]
pub open spec fn SecurityStateOnReturn(s: S) -> SecurityState;

} // verus!
