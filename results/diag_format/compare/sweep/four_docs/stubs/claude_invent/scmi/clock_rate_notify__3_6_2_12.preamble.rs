use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;

pub open spec fn IsValidClockDevice(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockRateNotifyEnabled(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockStateUnchangedExceptNotify(old_s: S, new_s: S, clock_id: UInt32) -> bool;

} // verus!
