use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub negotiated_protocol_version: int,
    pub supported_versions: Set<int>,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;

pub open spec fn IsProtocolVersionSupported(s: S, version: int) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S) -> int;

} // verus!
