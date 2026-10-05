use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type ProtocolVersion = u32;

pub type AgentId = u64;

pub struct S {
    pub negotiated_version: ProtocolVersion,
}

pub spec const SUCCESS: Int32 = 0;

pub spec const NOT_SUPPORTED: Int32 = 1;

#[allow(non_upper_case_globals)]
pub spec const version: ProtocolVersion = 1;

#[allow(non_upper_case_globals)]
pub spec const agent: AgentId = 0;

pub open spec fn PlatformSupportsProtocolVersion(v: ProtocolVersion) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion(a: AgentId) -> ProtocolVersion;

pub open spec fn MessagesComplyWithProtocolVersion(a: AgentId, v: ProtocolVersion) -> bool;

} // verus!
