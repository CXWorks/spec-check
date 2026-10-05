use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = -4;
pub spec const OUT_OF_RANGE: Int32 = -6;
pub spec const result: Int32 = 1;

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;

pub open spec fn IsValidRateIndex(s: S, clock_id: UInt32, rate_index: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn IsClockRateSegment(s: S, clock_id: UInt32, lowest: UInt64, highest: UInt64, step: UInt64) -> bool;

pub open spec fn LowestRate(rates: [UInt64; 4]) -> UInt64;

pub open spec fn HighestRate(rates: [UInt64; 4]) -> UInt64;

pub open spec fn StepSize(rates: [UInt64; 4]) -> UInt64;

pub open spec fn IsClockPhysicalRate(s: S, clock_id: UInt32, rate: UInt64) -> bool;

pub open spec fn RateHz(rates: [UInt64; 4], i: int) -> UInt64;

pub open spec fn ClockRateAtIndex(s: S, clock_id: UInt32, rate_index: UInt32) -> UInt64;

pub open spec fn NumRemainingRates(s: S, clock_id: UInt32, rate_index: UInt32, num_returned: int) -> UInt32;

pub open spec fn MaxTransportReturnRates() -> UInt32;

} // verus!
