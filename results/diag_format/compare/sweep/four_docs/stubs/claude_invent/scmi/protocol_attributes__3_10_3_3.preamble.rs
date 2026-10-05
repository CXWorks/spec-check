use vstd::prelude::*;
verus! {

pub struct S {
    pub num_power_capping_domains: u32,
}

pub open spec fn NumPowerCappingDomains(s: S) -> u32;

} // verus!
