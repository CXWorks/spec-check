use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0i32;
pub const NOT_SUPPORTED: i32 = -1i32;
pub const NOT_FOUND: i32 = -3i32;

pub open spec fn ClockGetPermissionsSupported(s: S) -> bool;

pub open spec fn ClockIsValid(s: S, clock_id: u32) -> bool;

pub open spec fn AgentCanChangeClockState(s: S, clock_id: u32) -> bool;

pub open spec fn AgentCanChangeClockParent(s: S, clock_id: u32) -> bool;

pub open spec fn AgentCanChangeClockRate(s: S, clock_id: u32) -> bool;

} // verus!
