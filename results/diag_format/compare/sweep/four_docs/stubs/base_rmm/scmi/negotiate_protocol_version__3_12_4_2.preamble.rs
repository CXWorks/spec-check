use vstd::prelude::*;
verus! {

pub type RmiStatusCode = u64;
pub type ProtocolVersion = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: RmiStatusCode = 0;
pub spec const NOT_SUPPORTED: RmiStatusCode = 1;

pub spec const version: ProtocolVersion = 0;

pub open spec fn IsPlatformSupportedProtocolVersion(v: ProtocolVersion) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn NegotiatedProtocolVersion() -> ProtocolVersion;

} // verus!
