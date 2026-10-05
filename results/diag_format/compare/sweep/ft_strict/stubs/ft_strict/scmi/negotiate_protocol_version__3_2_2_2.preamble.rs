use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;

pub struct NegotiatedProtocolVersion {
    pub value: uint32,
}

pub struct S {
    pub negotiated_protocol_version: uint32,
    pub supported_versions: Set<uint32>,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = 1;
#[allow(non_upper_case_globals)]
pub const result: int32 = 2;

pub open spec fn IsPlatformSupportedProtocolVersion(s: S, version: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

#[allow(non_snake_case)]
pub open spec fn NegotiatedProtocolVersion(s: S) -> uint32;

} // verus!
