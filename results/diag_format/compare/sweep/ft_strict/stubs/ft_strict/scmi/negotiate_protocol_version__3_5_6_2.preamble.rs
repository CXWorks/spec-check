use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub type Agent = u64;

pub struct S {
    pub negotiated_versions: Map<Agent, UInt32>,
    pub supported_versions: Set<UInt32>,
}

pub struct NegotiatedProtocolVersion {
    pub version: UInt32,
}

pub const SUCCESS: Int32 = 0;

pub const NOT_SUPPORTED: Int32 = -1;

pub spec const result: bool = true;

pub spec const agent: Agent = 0;

pub open spec fn IsProtocolVersionSupported(s: S, version: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S, ag: Agent) -> UInt32;

pub open spec fn CommandsResponsesNotificationsComplyWithVersion(s: S, ag: Agent, version: UInt32) -> bool;

} // verus!
