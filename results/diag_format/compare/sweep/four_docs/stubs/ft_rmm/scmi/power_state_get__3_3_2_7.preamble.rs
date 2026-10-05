use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;

pub struct PowerDomain {
    pub power_state: UInt32,
}

pub struct S {
    pub power_domains: Seq<PowerDomain>,
}

pub open spec fn IsValidPowerDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsDevicePowerDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn PowerDomainAt(s: S, domain_id: UInt32) -> PowerDomain;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

} // verus!
