use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type RmiStatusCode = u64;

pub type NegotiatedProtocolVersion = u32;

pub struct S {
    pub negotiated_version: u32,
}

pub const SUCCESS: RmiStatusCode = 0;
pub const NOT_SUPPORTED: RmiStatusCode = 1;

pub open spec fn PlatformSupportsProtocolVersion(s: S, version: UInt32) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn NegotiatedProtocolVersion(s: S) -> UInt32;

} // verus!
