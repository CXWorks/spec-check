use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct Agent {
    pub id: u64,
}

pub struct NegotiatedProtocolVersion {
    pub version: UInt32,
}

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = 1;
pub spec const result: Int32 = 2;

pub open spec fn PlatformSupportsProtocolVersion(s: S, version: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S, agent: Agent) -> UInt32;

pub open spec fn AllSubsequentMessagesComplyWithVersion(s: S, agent: Agent, version: UInt32) -> bool;

} // verus!
