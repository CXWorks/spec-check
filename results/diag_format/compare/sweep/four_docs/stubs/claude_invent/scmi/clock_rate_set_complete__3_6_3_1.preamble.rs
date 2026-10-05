use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const DENIED: i32 = -3;

pub open spec fn ClockRateSetSucceeded(old_s: S, new_s: S, clock_id: u32) -> bool;

pub open spec fn ClockRate(s: S, clock_id: u32) -> int;

pub open spec fn ClockHasOtherUsers(s: S, clock_id: u32) -> bool;

} // verus!
