use vstd::prelude::*;

verus! {

pub type RmiStatusCode = u64;

pub type UInt32 = u32;

pub type UInt64 = u64;

pub type AgentId = u32;

pub struct S {
    pub clocks: Seq<u32>,
    pub agents: Seq<u32>,
}

pub spec const SUCCESS: RmiStatusCode = 0;

pub spec const NOT_FOUND: RmiStatusCode = 1;

pub spec const clock_id: UInt32 = 0;

pub spec const notify_enable: UInt64 = 0;

pub open spec fn IsValidClockId(s: S, id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn Bits64(value: UInt64, hi: int, lo: int) -> int;

pub open spec fn CallingAgent() -> AgentId;

pub open spec fn RateChangeRequestedNotifyEnabled(s: S, agent: AgentId, id: UInt32) -> bool;

} // verus!
