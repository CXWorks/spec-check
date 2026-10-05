use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type RmiStatusCode = u64;

pub const SUCCESS: RmiStatusCode = 0;
pub const NOT_SUPPORTED: RmiStatusCode = 1;

#[allow(non_upper_case_globals)]
pub spec const version: UInt64 = 1;

pub struct S {
    pub dummy: UInt64,
}

pub open spec fn IsSupportedProtocolVersion(v: UInt64) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn NegotiatedProtocolVersion() -> UInt64;

} // verus!
