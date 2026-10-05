use vstd::prelude::*;
verus! {

pub type UInt32 = int;
pub type Int32 = int;
pub type CallerId = u64;

pub struct S {
    pub ffa_implemented: bool,
    pub negotiated: Map<CallerId, Int32>,
}

pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const INVALID_PARAMETER: Int32 = -2;
pub spec const NULL_VERSION: Int32 = -3;

#[allow(non_upper_case_globals)]
pub spec const caller: CallerId = 7;

pub open spec fn CalleeImplementsFfa(s: S) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn Bits(x: int, hi: int, lo: int) -> int;
pub open spec fn IsSpmcCallerAtSeparateEl(s: S, c: CallerId) -> bool;
pub open spec fn FfaInUse(s: S, c: CallerId) -> bool;
pub open spec fn VersionLess(a: int, b: int) -> bool;
pub open spec fn PrevNegotiatedVersion(s: S, c: CallerId) -> Int32;
pub open spec fn DowngradeAllowed(s: S, c: CallerId) -> bool;
pub open spec fn OnlyIncompatibleLowerVersions(s: S, v: UInt32) -> bool;
pub open spec fn OnlyIncompatibleHigherVersions(s: S, v: UInt32) -> bool;
pub open spec fn OnlyIncompatibleHigherAndLowerVersions(s: S, v: UInt32) -> bool;
pub open spec fn HighestIncompatibleVersion(s: S, v: UInt32) -> Int32;
pub open spec fn LowestIncompatibleVersion(s: S, v: UInt32) -> Int32;
pub open spec fn NegotiationSucceeds(s: S, c: CallerId, v: UInt32) -> bool;
pub open spec fn IsCompatible(a: int, b: int) -> bool;
pub open spec fn NegotiatedVersion(s: S, c: CallerId) -> Int32;
pub open spec fn SpmcStartNegotiatedVersion() -> Int32;
pub open spec fn HasCompatibleVersion(s: S, v: UInt32) -> bool;

} // verus!
