use vstd::prelude::*;
verus! {

pub type Int32 = i32;

pub type ProtocolVersion = u32;

pub struct S {
    pub negotiated_version: ProtocolVersion,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = 1;

pub const version: ProtocolVersion = 2;

pub open spec fn IsProtocolVersionSupported(v: ProtocolVersion) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion() -> ProtocolVersion;

pub open spec fn SubsequentMessagesComplyWithVersion(v: ProtocolVersion) -> bool;

} // verus!
