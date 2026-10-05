use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub struct S {
    pub agent_protocol_version: UInt32,
}

pub spec const SUCCESS: Int32 = 0;

pub spec const NOT_SUPPORTED: Int32 = 1;

pub spec const version: UInt32 = 0x00010002;

pub open spec fn PlatformSupportsProtocolVersion(s: S, v: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn AgentNegotiatedProtocolVersion(s: S) -> UInt32;

} // verus!
