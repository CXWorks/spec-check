use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const NOT_FOUND: i32 = -4;
pub const DENIED: i32 = -3;

pub open spec fn IsValidVoltageDomain(s: S, domain_id: u32) -> bool;

pub open spec fn IsVoltageConfigGetSupported(s: S, domain_id: u32) -> bool;

pub open spec fn IsAgentAllowedVoltageConfigGet(s: S, domain_id: u32) -> bool;

pub open spec fn VoltageDomainMode(s: S, domain_id: u32) -> u32;

} // verus!
