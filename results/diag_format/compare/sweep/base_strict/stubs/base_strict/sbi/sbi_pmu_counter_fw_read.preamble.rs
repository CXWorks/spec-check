use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn IsHardwareCounter(counter_idx: u64) -> bool;

pub open spec fn IsValidCounter(counter_idx: u64) -> bool;

pub open spec fn ResultEqual(a: i64, b: i64) -> bool;

pub open spec fn IsRv32() -> bool;

pub open spec fn FirmwareCounterValue(counter_idx: u64) -> u64;

pub open spec fn Bits(x: u64, hi: int, lo: int) -> u64;

} // verus!
