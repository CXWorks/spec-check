use vstd::prelude::*;
verus! {

pub struct S {
    pub clock_count: u32,
    pub clock_rates: Seq<Seq<u64>>,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;
pub const OUT_OF_RANGE: i32 = -6;

pub open spec fn ClockExists(s: S, clock_id: u32) -> bool;

pub open spec fn ClockRateIndexInRange(s: S, clock_id: u32, rate_index: u32) -> bool;

} // verus!
