use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;

pub struct NegotiatedProtocolVersion {
    pub version: uint32,
}

pub struct S {
    pub negotiated_version: uint32,
    pub supported_versions: Set<uint32>,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = 1;

pub open spec fn result_value() -> int32;

pub spec const result: int32 = result_value();

pub open spec fn IsSupportedProtocolVersion(s: S, version: uint32) -> bool;

pub open spec fn ResultEqual(status: int32, expected: int32) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S) -> uint32;

} // verus!
