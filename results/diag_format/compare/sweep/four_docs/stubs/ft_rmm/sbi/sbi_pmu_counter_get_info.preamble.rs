use vstd::prelude::*;
verus! {

pub type long = i64;
pub type unsigned_long = u64;
pub type UInt64 = u64;
pub type Int64 = i64;

pub const XLEN: u64 = 64;

pub struct S {
    pub num_counters: u64,
    pub num_hw_counters: u64,
    pub num_fw_counters: u64,
}

pub open spec fn IsFirmwareCounter(counter_idx: u64) -> bool;

pub open spec fn CounterCsrNumber(counter_idx: u64) -> u64;

pub open spec fn CounterBitWidth(counter_idx: u64) -> u64;

} // verus!
