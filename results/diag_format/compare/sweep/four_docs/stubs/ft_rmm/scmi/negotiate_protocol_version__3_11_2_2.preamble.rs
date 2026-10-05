use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct NegotiatedProtocolVersion {
    pub value: UInt32,
}

pub struct S {
    pub negotiated_version: UInt32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = 1;
#[allow(non_upper_case_globals)]
pub spec const result: Int32 = 2;

pub open spec fn IsProtocolVersionSupportedByPlatform(s: S, version: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

#[allow(non_snake_case)]
pub open spec fn NegotiatedProtocolVersion(s: S) -> UInt32;

} // verus!
