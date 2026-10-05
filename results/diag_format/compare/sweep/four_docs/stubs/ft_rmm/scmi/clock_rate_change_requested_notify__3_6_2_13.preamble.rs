use vstd::prelude::*;
verus! {

pub type uint32 = Seq<u32>;

pub type int32 = i32;

pub type AgentId = u32;

pub struct S {
    pub clocks: Seq<u32>,
    pub notify_enabled: Map<(AgentId, uint32), u32>,
}

pub spec const SUCCESS: int32 = 0;

pub spec const NOT_FOUND: int32 = (-4int) as i32;

pub spec const result: int32 = 1;

pub spec const calling_agent: AgentId = 0;

pub open spec fn IsValidClockId(s: S, clock_id: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn RateChangeRequestedNotifyEnabled(s: S, agent: AgentId, clock_id: uint32) -> u32;

} // verus!
