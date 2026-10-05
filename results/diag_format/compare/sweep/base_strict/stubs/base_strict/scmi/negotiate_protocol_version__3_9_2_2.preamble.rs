use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type ProtocolVersion = u32;

pub struct S {
    pub negotiated_version: ProtocolVersion,
}

pub const SUCCESS: Int32 = 0;

pub const NOT_SUPPORTED: Int32 = -1;

#[allow(non_upper_case_globals)]
pub const version: ProtocolVersion = 1;

pub open spec fn PlatformSupportsProtocolVersion(v: ProtocolVersion) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn NegotiatedProtocolVersion() -> ProtocolVersion;

} // verus!
