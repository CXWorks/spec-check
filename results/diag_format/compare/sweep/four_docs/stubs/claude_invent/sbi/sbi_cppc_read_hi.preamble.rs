use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct SbiRet {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_DENIED: i64 = -4;

pub open spec fn CppcRegIsReserved(s: S, reg_id: UInt32) -> bool;

pub open spec fn CppcRegIsImplemented(s: S, reg_id: UInt32) -> bool;

pub open spec fn CppcRegIsWriteOnly(s: S, reg_id: UInt32) -> bool;

pub open spec fn SupervisorXlen(s: S) -> u32;

pub open spec fn CppcRegValue(s: S, reg_id: UInt32) -> u64;

} // verus!
