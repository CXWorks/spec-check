use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub type Agent = u32;

pub struct S {
    pub placeholder: u32,
}

pub const SUCCESS: Int32 = 0;

pub const NOT_SUPPORTED: Int32 = 1;

pub const result: Int32 = 2;

pub const agent: Agent = 0;

pub open spec fn PlatformSupportsProtocolVersion(s: S, version: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S, a: Agent) -> UInt32;

pub open spec fn AllSubsequentMessagesComplyWithVersion(s: S, a: Agent, version: UInt32) -> bool;

} // verus!
