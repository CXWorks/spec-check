use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub struct VoltageDomain {
    pub config: u32,
}

pub struct VoltageDomains {
    pub m: Map<u32, VoltageDomain>,
}

impl VoltageDomains {
    pub uninterp spec fn contains(self, k: u32) -> bool;

    pub uninterp spec fn spec_index(self, k: u32) -> VoltageDomain;
}

pub struct S {
    pub domain_id: u32,
    pub voltage_domains: VoltageDomains,
}

} // verus!
