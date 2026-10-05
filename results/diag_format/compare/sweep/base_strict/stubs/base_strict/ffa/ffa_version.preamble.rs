use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type CallerId = u64;

pub struct S {
    pub negotiated_version: Int32,
    pub ffa_in_use: bool,
}

pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const INVALID_PARAMETER: Int32 = (-2int) as i32;
pub spec const NULL_VERSION: Int32 = (-3int) as i32;

pub spec const version: Int32 = 65537;
pub spec const flags: Int32 = 0;
pub spec const caller: CallerId = 1;

pub open spec fn CalleeImplementsFfa() -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn Bits64(x: Int32, hi: int, lo: int) -> int;
pub open spec fn ReservedResultRegsAreZero() -> bool;
pub open spec fn IsSpmcCallerAtSeparateEl(c: CallerId) -> bool;
pub open spec fn FfaInUse(c: CallerId) -> bool;
pub open spec fn VersionLess(a: Int32, b: Int32) -> bool;
pub open spec fn PrevNegotiatedVersion(c: CallerId) -> Int32;
pub open spec fn DowngradeAllowed(c: CallerId) -> bool;
pub open spec fn OnlyIncompatibleLowerVersions(v: Int32) -> bool;
pub open spec fn OnlyIncompatibleHigherVersions(v: Int32) -> bool;
pub open spec fn OnlyIncompatibleHigherAndLowerVersions(v: Int32) -> bool;
pub open spec fn HighestIncompatibleVersion(v: Int32) -> Int32;
pub open spec fn LowestIncompatibleVersion(v: Int32) -> Int32;
pub open spec fn NegotiationSucceeds(c: CallerId, v: Int32) -> bool;
pub open spec fn IsCompatible(v: Int32, r: Int32) -> bool;
pub open spec fn NegotiatedVersion(c: CallerId) -> Int32;
pub open spec fn SpmcStartNegotiatedVersion() -> Int32;
pub open spec fn HasCompatibleVersion(v: Int32) -> bool;

} // verus!
