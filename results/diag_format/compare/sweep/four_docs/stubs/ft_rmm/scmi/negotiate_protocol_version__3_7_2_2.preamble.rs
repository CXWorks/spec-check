use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type NegotiatedProtocolVersion = u32;

pub struct S {
    pub supported_versions: Set<u32>,
    pub negotiated_version: u32,
}

pub spec const SUCCESS: Int32 = 0i32;
pub spec const NOT_SUPPORTED: Int32 = -1i32;

#[allow(non_upper_case_globals)]
pub spec const result: bool = true;

#[allow(non_upper_case_globals)]
pub spec const NegotiatedProtocolVersion: UInt32 = 0u32;

pub open spec fn PlatformSupportsProtocolVersion(s: S, version: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

} // verus!
