use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct SbiRet {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn IsFirmwareCounter(s: S, counter_idx: UInt64) -> bool;

pub open spec fn IsRv32(s: S) -> bool;

pub open spec fn FirmwareCounterValue(s: S, counter_idx: UInt64) -> u64;

} // verus!
