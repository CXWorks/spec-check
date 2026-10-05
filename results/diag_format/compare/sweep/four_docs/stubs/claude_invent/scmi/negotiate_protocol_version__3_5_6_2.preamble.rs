use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub negotiated_protocol_version: UInt32,
    pub supported_versions: Seq<UInt32>,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = 1;

pub open spec fn IsProtocolVersionSupported(s: S, version: UInt32) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S) -> UInt32;

} // verus!
