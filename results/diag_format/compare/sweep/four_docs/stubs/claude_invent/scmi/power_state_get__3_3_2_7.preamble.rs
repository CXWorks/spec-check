use vstd::prelude::*;
verus! {

pub struct S {
    pub power_domains: Map<u32, u32>,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -2;

pub open spec fn IsValidPowerDomain(s: S, domain_id: u32) -> bool;

pub open spec fn PowerDomainState(s: S, domain_id: u32) -> u32;

} // verus!
