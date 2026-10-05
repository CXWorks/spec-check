use vstd::prelude::*;

verus! {

pub type int32 = i32;

pub type uint32 = u32;

pub struct S {
    pub negotiated_version: uint32,
}

pub const SUCCESS: int32 = 0;

pub const NOT_SUPPORTED: int32 = -1;

#[allow(non_upper_case_globals)]
pub const version: uint32 = 0x10002;

pub open spec fn IsPlatformSupportedProtocolVersion(v: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn NegotiatedProtocolVersion() -> uint32;

} // verus!
