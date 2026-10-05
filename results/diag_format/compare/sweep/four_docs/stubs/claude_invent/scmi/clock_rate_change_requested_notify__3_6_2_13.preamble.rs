use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub clock_count: u32,
    pub notify_enabled: Map<u32, bool>,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;

pub open spec fn ClockIdIsValid(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockRateChangeRequestedNotifyEnabled(s: S, clock_id: UInt32) -> bool;

} // verus!
