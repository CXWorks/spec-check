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

pub open spec fn VoltageConfigSupported(s: S, domain_id: u32, mode: u32) -> bool;

pub open spec fn AgentMaySetVoltageConfig(s: S, domain_id: u32) -> bool;

pub open spec fn VoltageConfigSetRequestSupported(s: S, domain_id: u32, config: u32) -> bool;

pub open spec fn VoltageDomainMode(s: S, domain_id: u32) -> u32;

} // verus!
