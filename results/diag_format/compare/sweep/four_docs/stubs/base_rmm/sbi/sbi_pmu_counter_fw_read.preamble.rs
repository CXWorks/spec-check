use vstd::prelude::*;
verus! {

pub type long = i64;
pub type unsigned_long = u64;
pub type ulong = u64;

pub struct S {
    pub counters: Seq<u64>,
    pub hart_id: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn IsValidCounter(counter_idx: u64) -> bool;

pub open spec fn IsHardwareCounter(counter_idx: u64) -> bool;

pub open spec fn ResultEqual(error: i64, code: i64) -> bool;

pub open spec fn FirmwareCounterValue(counter_idx: u64) -> u64;

pub open spec fn IsRV32() -> bool;

} // verus!
