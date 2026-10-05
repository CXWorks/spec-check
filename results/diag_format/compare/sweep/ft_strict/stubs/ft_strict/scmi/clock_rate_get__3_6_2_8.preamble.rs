use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockIsDisabled(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockCurrentRate(s: S, clock_id: UInt32) -> int;

pub open spec fn ClockRateOnReenable(s: S, clock_id: UInt32) -> int;

} // verus!
