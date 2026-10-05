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

#[allow(non_upper_case_globals)]
pub const result: Int32 = 1;

#[allow(non_upper_case_globals)]
pub const agent: AgentId = 0;

pub open spec fn PlatformSupportsProtocolVersion(s: S, version: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S, agent_id: AgentId) -> UInt32;

pub open spec fn MessagesComplyWithProtocolVersion(s: S, agent_id: AgentId, version: UInt32) -> bool;

} // verus!
