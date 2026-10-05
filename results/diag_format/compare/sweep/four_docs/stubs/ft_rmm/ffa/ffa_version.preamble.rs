use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = u32;

pub struct Endpoint {
    pub id: u32,
}

pub struct S {
    pub dummy: u32,
}

pub const NOT_SUPPORTED: Int32 = 0xFFFF_FFFFu32;
pub const INVALID_PARAMETER: Int32 = 0xFFFF_FFFEu32;
pub const NULL_VERSION: Int32 = 0x7FFF_FFFFu32;

pub open spec fn spmc_of(s: S) -> Endpoint;

pub spec const SPMC: spec_fn(S) -> Endpoint = |s: S| spmc_of(s);

pub open spec fn caller(s: S) -> Endpoint;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn CalleeImplementsFfa(s: S) -> bool;

pub open spec fn IsValidVersionQueryType(t: UInt32) -> bool;

pub open spec fn CallerIsSpmcAtSeparateEl(s: S) -> bool;

pub open spec fn FfaInUse(e: Endpoint) -> bool;

pub open spec fn VersionLessThan(a: UInt32, b: UInt32) -> bool;

pub open spec fn NegotiatedVersion(e: Endpoint) -> UInt32;

pub open spec fn CalleeAllowsDowngrade(s: S) -> bool;

pub open spec fn CalleeImplementsCompatibleVersion(s: S, v: UInt32) -> bool;

pub open spec fn IsCompatible(a: Int32, b: UInt32) -> bool;

pub open spec fn CalleeIncompatibleVersionsOnlyLower(s: S, v: UInt32) -> bool;

pub open spec fn CalleeIncompatibleVersionsOnlyHigher(s: S, v: UInt32) -> bool;

pub open spec fn CalleeIncompatibleVersionsHigherAndLower(s: S, v: UInt32) -> bool;

pub open spec fn HighestIncompatibleVersion(s: S, v: UInt32) -> UInt32;

pub open spec fn LowestIncompatibleVersion(s: S, v: UInt32) -> UInt32;

pub open spec fn NegotiatedVersionAtStart(s: S, who: spec_fn(S) -> Endpoint) -> UInt32;

pub open spec fn VersionNegotiated(e: Endpoint) -> bool;

} // verus!
