use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct Array<T> {
    pub data: Seq<T>,
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;
pub const OUT_OF_RANGE: Int32 = -6;

pub open spec fn ClockExists(clock_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidRateIndex(clock_id: UInt32, rate_index: UInt32) -> bool;

pub open spec fn Bits64(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn IsClockRateSegment(clock_id: UInt32, lowest: UInt64, highest: UInt64, step: UInt64) -> bool;

pub open spec fn LowestRate(rates: Array<UInt64>) -> UInt64;

pub open spec fn HighestRate(rates: Array<UInt64>) -> UInt64;

pub open spec fn StepSize(rates: Array<UInt64>) -> UInt64;

pub open spec fn IsClockPhysicalRate(clock_id: UInt32, rate: UInt64) -> bool;

pub open spec fn RateHz<T>(rates: Array<UInt64>, i: T) -> UInt64;

pub open spec fn ClockRateAtIndex(clock_id: UInt32, rate_index: UInt32) -> UInt64;

pub open spec fn NumRemainingRates(clock_id: UInt32, rate_index: UInt32, num_returned: UInt32) -> UInt32;

pub open spec fn MaxTransportReturnRates() -> UInt32;

} // verus!
