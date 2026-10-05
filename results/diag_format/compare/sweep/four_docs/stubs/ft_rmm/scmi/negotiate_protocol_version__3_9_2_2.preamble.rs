use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type NegotiatedProtocolVersion = u32;

pub struct S {
    pub negotiated_version: u32,
    pub supported_versions: Seq<u32>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = 1;
#[allow(non_upper_case_globals)]
pub const result: Int32 = 2;

pub open spec fn IsSupportedProtocolVersion(s: S, version: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S) -> UInt32;

} // verus!
