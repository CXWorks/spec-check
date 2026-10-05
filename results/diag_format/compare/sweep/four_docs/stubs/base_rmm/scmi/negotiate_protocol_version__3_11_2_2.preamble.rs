use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub negotiated_protocol_version: UInt32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = 1;

#[allow(non_upper_case_globals)]
pub spec const version: UInt32 = 2;

pub open spec fn IsProtocolVersionSupportedByPlatform(v: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion() -> UInt32;

} // verus!
