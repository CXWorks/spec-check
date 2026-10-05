use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NOT_FOUND: Int32 = -4;
pub const DENIED: Int32 = -3;

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;
pub open spec fn IsClockParentGetSupported(s: S, clock_id: UInt32) -> bool;
pub open spec fn CallingAgent() -> AgentId;
pub open spec fn IsAgentAllowedToGetClockParent(s: S, agent: AgentId, clock_id: UInt32) -> bool;
pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;
pub open spec fn ClockParent(s: S, clock_id: UInt32) -> UInt32;

} // verus!
