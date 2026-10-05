use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -6;

pub struct PowerDomain {
    pub power_state: UInt32,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn IsValidPowerDomain(domain_id: UInt32) -> bool;

pub open spec fn IsDevicePowerDomain(domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PowerDomainAt(domain_id: UInt32) -> PowerDomain;

} // verus!
