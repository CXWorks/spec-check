use vstd::prelude::*;
verus! {

pub type int32 = i64;

pub struct PerformanceDomain {
    pub name: [u8; 64],
}

pub struct PerformanceDomains {
    pub domains: Seq<PerformanceDomain>,
}

impl PerformanceDomains {
    pub uninterp spec fn spec_index(self, i: usize) -> PerformanceDomain;
}

pub struct S {
    pub domain_id: u32,
    pub performance_domains: PerformanceDomains,
}

} // verus!
