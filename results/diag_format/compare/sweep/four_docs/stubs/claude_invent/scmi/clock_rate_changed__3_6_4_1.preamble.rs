use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsValidClockId(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockRateNotifySubscribed(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockRateChangeTransitionCompleted(old_s: S, new_s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockRateHz(s: S, clock_id: UInt32) -> int;

pub open spec fn ClockRateChangeCausedBy(old_s: S, new_s: S, clock_id: UInt32, agent_id: UInt32) -> bool;

} // verus!
