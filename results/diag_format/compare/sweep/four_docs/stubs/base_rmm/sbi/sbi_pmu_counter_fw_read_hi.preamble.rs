use vstd::prelude::*;

verus! {

pub type long = i64;
pub type unsigned = u64;
pub type ulong = u64;

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: long = 0;
pub const SBI_ERR_INVALID_PARAM: long = -3;

pub const XLEN: u64 = 64;

pub open spec fn IsHardwareCounter(counter_idx: u64) -> bool;

pub open spec fn IsValidCounter(counter_idx: u64) -> bool;

pub open spec fn ResultEqual(error: long, code: long) -> bool;

pub open spec fn FirmwareCounterValue(counter_idx: u64) -> u64;

} // verus!
