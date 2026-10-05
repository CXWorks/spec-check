use vstd::prelude::*;

verus! {

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const OUT_OF_RANGE: i32 = -3;
pub const NOT_FOUND: i32 = -4;
pub const DENIED: i32 = -5;

pub struct S {
    pub dummy: int,
}

pub open spec fn ClockExists(s: S, clock_id: u32) -> bool;

pub open spec fn IsValidSkipParents(s: S, clock_id: u32, skip_parents: u32) -> bool;

pub open spec fn ClockParentsAdvertisingSupported(s: S, clock_id: u32) -> bool;

pub open spec fn AgentAllowedToGetClockParents(s: S, clock_id: u32) -> bool;

pub open spec fn ClockNumPossibleParents(s: S, clock_id: u32) -> u32;

pub open spec fn ClockPossibleParentAt(s: S, clock_id: u32, index: int) -> u32;

} // verus!
