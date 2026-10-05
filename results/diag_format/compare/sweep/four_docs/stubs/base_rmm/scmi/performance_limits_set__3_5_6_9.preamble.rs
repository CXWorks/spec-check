use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct PerfDomain {
    pub range_max: UInt32,
    pub range_min: UInt32,
}

pub struct S {
    pub dummy: UInt32,
}

impl S {
    pub uninterp spec fn PerformanceDomain(self, did: UInt32) -> PerfDomain;
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const OUT_OF_RANGE: Int32 = -2;
pub const DENIED: Int32 = -3;

pub const domain_id: UInt32 = 1;
pub const range_max: UInt32 = 2;
pub const range_min: UInt32 = 3;

pub uninterp spec fn PerformanceDomainExists(s: S, did: UInt32) -> bool;
pub uninterp spec fn LimitsWithinDescribedLevels(s: S, did: UInt32, rmax: UInt32, rmin: UInt32) -> bool;
pub uninterp spec fn AgentMayChangePerformanceLimits(s: S, did: UInt32) -> bool;
pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub uninterp spec fn PerformanceDomain(s: S, did: UInt32) -> PerfDomain;

} // verus!
