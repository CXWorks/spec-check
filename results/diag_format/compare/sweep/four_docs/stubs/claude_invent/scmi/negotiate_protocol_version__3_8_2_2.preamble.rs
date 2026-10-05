use vstd::prelude::*;
verus! {

pub struct S {
    pub negotiated_protocol_version: u32,
    pub supported_mask: u64,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;

pub open spec fn IsProtocolVersionSupported(s: S, version: u32) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S) -> u32;

} // verus!
