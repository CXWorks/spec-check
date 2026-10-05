use vstd::prelude::*;

verus! {

pub struct S {
    pub calling_agent: u32,
    pub agent_count: u32,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -2;

pub open spec fn AgentIdIsValid(s: S, agent_id: u32) -> bool;

pub open spec fn CallingAgentId(s: S) -> u32;

pub open spec fn AgentName(s: S, agent_id: u32) -> Seq<u8>;

} // verus!
