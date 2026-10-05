use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type FfaInstance = u8;
pub type ExceptionLevel = u8;

pub const NON_SECURE_PHYSICAL: FfaInstance = 0;
pub const NON_SECURE_VIRTUAL: FfaInstance = 1;
pub const SECURE_PHYSICAL: FfaInstance = 2;
pub const SECURE_VIRTUAL: FfaInstance = 3;

pub const S_EL1: ExceptionLevel = 1;
pub const S_EL2: ExceptionLevel = 2;
pub const EL3: ExceptionLevel = 3;

pub const FFA_SPM_ID_GET: UInt32 = 0x84000085;

pub const FFA_SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;

pub struct S {
    pub ffa_instance: FfaInstance,
}

pub open spec fn IsImplementedAtInstance(func_id: UInt32, instance: FfaInstance) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn SpmcId() -> UInt16;

pub open spec fn SpmdId() -> UInt16;

pub open spec fn SpmcEl(s: S) -> ExceptionLevel;

} // verus!
