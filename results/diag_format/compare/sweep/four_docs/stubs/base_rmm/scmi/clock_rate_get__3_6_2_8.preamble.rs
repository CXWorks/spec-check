use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u64;
pub type UInt64 = u64;
pub type ClockId = u32;

pub struct S {
    pub clocks_dummy: int,
}

pub spec const clock_id: ClockId = 0;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -6;

pub open spec fn ClockExists(s: S, id: ClockId) -> bool;
pub open spec fn ClockIsEnabled(s: S, id: ClockId) -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn ClockCurrentRate(s: S, id: ClockId) -> u64;
pub open spec fn ClockRateOnReenable(s: S, id: ClockId) -> u64;

} // verus!
