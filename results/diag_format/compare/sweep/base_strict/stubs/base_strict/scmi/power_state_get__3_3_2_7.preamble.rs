use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub domain_id: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -3;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn IsValidPowerDomain(domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn ReportsCurrentPowerState(domain_id: UInt32, power_state: UInt32) -> bool;

pub open spec fn IsDevicePowerDomain(domain_id: UInt32) -> bool;

pub open spec fn IsDevicePowerStateEncoding(power_state: UInt32) -> bool;

} // verus!
