use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub clock_rates: Map<UInt32, UInt64>,
    pub clock_ids: Set<UInt32>,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = (-3int) as i32;

pub uninterp spec fn ClockExists(s: S, clock_id: UInt32) -> bool;

pub uninterp spec fn ClockRate(s: S, clock_id: UInt32) -> UInt64;

} // verus!
