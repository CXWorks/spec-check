use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub type NegotiatedProtocolVersion = u32;

pub struct S {
    pub supported_versions: Set<u32>,
    pub negotiated_version: u32,
}

pub const SUCCESS: Int32 = 0;

pub const NOT_SUPPORTED: Int32 = 1;

#[allow(non_upper_case_globals)]
pub const result: bool = true;

pub open spec fn PlatformSupportsProtocolVersion(s: S, version: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

#[allow(non_snake_case)]
pub open spec fn NegotiatedProtocolVersion(s: S) -> UInt32;

} // verus!
