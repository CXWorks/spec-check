use vstd::prelude::*;

verus! {

pub type long = i64;
pub type unsigned_long = u64;

pub struct S {
    pub counters: Seq<u64>,
}

pub spec const SBI_SUCCESS: i64 = 0;
pub spec const SBI_ERR_INVALID_PARAM: i64 = -3;

pub spec const XLEN: u64 = 64;

pub open spec fn IsHardwareCounter(s: S, counter_idx: u64) -> bool;

pub open spec fn IsValidCounter(s: S, counter_idx: u64) -> bool;

pub open spec fn ResultEqual(a: i64, b: i64) -> bool;

pub open spec fn FirmwareCounterValue(s: S, counter_idx: u64) -> u64;

} // verus!
