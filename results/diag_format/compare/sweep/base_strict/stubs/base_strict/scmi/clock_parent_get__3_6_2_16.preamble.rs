use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1) as Int32;
pub spec const DENIED: Int32 = (-3) as Int32;
pub spec const NOT_FOUND: Int32 = (-4) as Int32;

pub spec const clock_id: UInt32 = 0;

pub uninterp spec fn ClockExists(s: S, clock_id: UInt32) -> bool;

pub uninterp spec fn IsClockParentGetSupported(s: S, clock_id: UInt32) -> bool;

pub uninterp spec fn CallingAgent() -> UInt32;

pub uninterp spec fn IsAgentAllowedToGetClockParent(agent_id: UInt32, clock_id: UInt32) -> bool;

pub uninterp spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub uninterp spec fn ClockParent(clock_id: UInt32) -> UInt32;

} // verus!
