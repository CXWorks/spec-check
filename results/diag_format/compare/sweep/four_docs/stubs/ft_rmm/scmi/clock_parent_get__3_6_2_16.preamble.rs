use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub clock_ids: Set<uint32>,
    pub supported: Set<uint32>,
    pub parents: Map<uint32, uint32>,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const DENIED: int32 = -3;
pub const NOT_FOUND: int32 = -4;

#[allow(non_upper_case_globals)]
pub const calling_agent: AgentId = 0;

#[allow(non_upper_case_globals)]
pub const result: int32 = -100;

pub open spec fn ClockExists(s: S, clock_id: uint32) -> bool;

pub open spec fn IsRequestSupported(s: S, clock_id: uint32) -> bool;

pub open spec fn AgentMayGetClockParent(s: S, agent: AgentId, clock_id: uint32) -> bool;

pub open spec fn ResultEqual(status: int32, expected: int32) -> bool;

pub open spec fn ClockParent(s: S, clock_id: uint32) -> uint32;

} // verus!
