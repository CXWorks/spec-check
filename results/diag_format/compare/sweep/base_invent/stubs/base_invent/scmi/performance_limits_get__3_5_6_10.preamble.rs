use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub type UInt64 = u64;

pub struct PerformanceDomain {
    pub id: UInt64,
}

pub struct PerformanceDomains {
    pub domains: Seq<PerformanceDomain>,
}

impl PerformanceDomains {
    pub open spec fn spec_index(self, i: UInt64) -> PerformanceDomain;

    pub open spec fn contains(self, d: &PerformanceDomain) -> bool;
}

pub struct S {
    pub performance_domains: PerformanceDomains,
    pub domain_id: UInt64,
    pub performance_limits_range_min: UInt64,
    pub performance_limits_range_max: UInt64,
}

} // verus!
