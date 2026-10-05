use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct PerformanceDomainInfo {
    pub range_max: UInt32,
    pub range_min: UInt32,
}

pub struct S {
    pub domains: Map<UInt32, PerformanceDomainInfo>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;
pub const OUT_OF_RANGE: Int32 = -2;
pub const DENIED: Int32 = -3;
#[allow(non_upper_case_globals)]
pub const result: Int32 = -100;

pub open spec fn PerformanceDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn LimitsWithinDescribedLevels(s: S, domain_id: UInt32, range_max: UInt32, range_min: UInt32) -> bool;

pub open spec fn AgentMayChangePerformanceLimits(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PerformanceDomain(s: S, domain_id: UInt32) -> PerformanceDomainInfo;

} // verus!
