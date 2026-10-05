use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub type AgentHandle = u64;

pub struct S {
    pub negotiated_version: UInt32,
}

pub const SUCCESS: Int32 = 0;

pub const NOT_SUPPORTED: Int32 = 1;

pub const version: UInt32 = 2;

pub const agent: AgentHandle = 3;

pub open spec fn IsPlatformSupportedProtocolVersion(v: UInt32) -> bool;

pub open spec fn ResultEqual(r: Int32, code: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion(a: AgentHandle) -> UInt32;

} // verus!
