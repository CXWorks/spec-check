use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u32,
}

pub const FFA_SUCCESS: u32 = 0x84000061u32;
pub const FFA_SPM_ID_GET: u32 = 0x84000085u32;

pub const NOT_SUPPORTED: u32 = 0xFFFFFFFFu32;

pub const NON_SECURE_PHYSICAL: u32 = 0u32;
pub const NON_SECURE_VIRTUAL: u32 = 1u32;
pub const SECURE_PHYSICAL: u32 = 2u32;
pub const SECURE_VIRTUAL: u32 = 3u32;

pub const S_EL1: u32 = 11u32;
pub const S_EL2: u32 = 12u32;
pub const EL3: u32 = 13u32;

pub open spec fn IsImplementedAtInstance(function_id: u32, instance: u32) -> bool;

pub open spec fn CallerInstance() -> u32;

pub open spec fn ResultEqual(result: u32, code: u32) -> bool;

pub open spec fn Bits(value: u32, hi: int, lo: int) -> u32;

pub open spec fn SpmcId() -> u32;

pub open spec fn SpmdId() -> u32;

pub open spec fn SpmcExceptionLevel() -> u32;

} // verus!
