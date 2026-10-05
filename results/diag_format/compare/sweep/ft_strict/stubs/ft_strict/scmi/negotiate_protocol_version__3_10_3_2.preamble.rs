use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const result: Int32 = -2;

pub open spec fn PlatformSupportsProtocolVersion(s: S, protocol_id: UInt32, version: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn AgentNegotiatedProtocolVersion(s: S, protocol_id: UInt32) -> UInt32;

} // verus!
