use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;
pub const OUT_OF_RANGE: Int32 = -2;

pub const clock_id: UInt32 = 0;
pub const rate_index: UInt32 = 1;
pub const N: UInt32 = 2;

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;

pub open spec fn IsValidRateIndex(s: S, clock_id: UInt32, rate_index: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

} // verus!
