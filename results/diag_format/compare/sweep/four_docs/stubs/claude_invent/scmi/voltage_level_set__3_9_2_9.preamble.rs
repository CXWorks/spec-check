use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -3;
pub const NOT_FOUND: i32 = -4;

pub open spec fn VoltageDomainExists(s: S, domain_id: u32) -> bool;

pub open spec fn VoltageLevelSetRequestSupported(s: S, domain_id: u32, flags: u32) -> bool;

pub open spec fn AgentAllowedToSetVoltageLevel(s: S, domain_id: u32) -> bool;

pub open spec fn VoltageLevelSupported(s: S, domain_id: u32, voltage_level: i32) -> bool;

pub open spec fn VoltageLevel(s: S, domain_id: u32) -> i32;

pub open spec fn VoltageLevelSetQueued(s: S, domain_id: u32, voltage_level: i32) -> bool;

} // verus!
