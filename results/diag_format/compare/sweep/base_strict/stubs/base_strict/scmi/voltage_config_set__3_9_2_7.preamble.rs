use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub domain_id_field: u32,
    pub config_field: u32,
}

pub struct VoltageDomainState {
    pub mode: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

pub const VOLTAGE_CONFIG_SET: u32 = 0x5;

pub open spec fn domain_id(s: S) -> u32;

pub open spec fn config(s: S) -> u32;

pub open spec fn Bits(value: u32, hi: int, lo: int) -> u32;

pub open spec fn VoltageDomainExists(domain_id: u32) -> bool;

pub open spec fn VoltageDomainSupportsMode(domain_id: u32, mode: u32) -> bool;

pub open spec fn IsRequestSupported(message_id: u32) -> bool;

pub open spec fn CallingAgent() -> u32;

pub open spec fn AgentMaySetVoltageConfig(agent_id: u32, domain_id: u32) -> bool;

pub open spec fn VoltageDomain(domain_id: u32) -> VoltageDomainState;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

} // verus!
