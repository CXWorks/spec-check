use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type ProtocolVersion = u32;
pub type AgentId = u64;

pub struct S {
    pub negotiated_versions: Map<AgentId, ProtocolVersion>,
    pub supported_versions: Set<ProtocolVersion>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;

#[allow(non_upper_case_globals)]
pub const version: ProtocolVersion = 1;

#[allow(non_upper_case_globals)]
pub const agent: AgentId = 0;

pub open spec fn PlatformSupportsProtocolVersion(s: S, v: ProtocolVersion) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S, a: AgentId) -> ProtocolVersion;

pub open spec fn AllSubsequentMessagesComplyWithVersion(s: S, a: AgentId, v: ProtocolVersion) -> bool;

} // verus!
