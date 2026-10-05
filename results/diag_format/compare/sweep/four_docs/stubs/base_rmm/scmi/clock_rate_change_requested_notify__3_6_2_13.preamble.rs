use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub type AgentId = u32;

pub type ClockId = u32;

pub struct S {
    pub calling_agent: AgentId,
    pub clock_id: ClockId,
}

pub const SUCCESS: int32 = 0;

pub const NOT_FOUND: int32 = -4;

pub open spec fn IsValidClockId(id: ClockId) -> bool;

pub open spec fn clock_id(s: S) -> ClockId;

pub open spec fn calling_agent(s: S) -> AgentId;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn RateChangeRequestedNotifyEnabled(agent: AgentId, id: ClockId) -> u32;

} // verus!
