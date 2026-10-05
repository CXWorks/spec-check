use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub type CounterInfo = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub struct S {
    pub num_counters: nat,
}

pub open spec fn Bits(value: CounterInfo, hi: int, lo: int) -> int;

pub open spec fn CounterTypeEncoding(s: S, counter_idx: UInt) -> int;

pub open spec fn IsHardwareCounter(s: S, counter_idx: UInt) -> bool;

pub open spec fn IsFirmwareCounter(s: S, counter_idx: UInt) -> bool;

pub open spec fn CounterCsrNumber(s: S, counter_idx: UInt) -> int;

pub open spec fn CounterBitWidth(s: S, counter_idx: UInt) -> int;

} // verus!
