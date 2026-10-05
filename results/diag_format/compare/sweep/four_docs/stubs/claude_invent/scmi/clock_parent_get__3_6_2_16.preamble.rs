use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const NOT_FOUND: i32 = -4;
pub const DENIED: i32 = -3;

pub open spec fn ClockExists(s: S, clock_id: u32) -> bool;
pub open spec fn ClockParentGetSupported(s: S, clock_id: u32) -> bool;
pub open spec fn AgentAllowedClockParentGet(s: S, clock_id: u32) -> bool;
pub open spec fn ClockParentOf(s: S, clock_id: u32) -> u32;

} // verus!
