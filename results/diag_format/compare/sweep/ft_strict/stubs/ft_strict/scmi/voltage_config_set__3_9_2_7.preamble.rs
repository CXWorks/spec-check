use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct VoltageDomainState {
    pub mode: UInt32,
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

pub const result: Int32 = 1000;

pub const VOLTAGE_CONFIG_SET: UInt32 = 5;

pub open spec fn VoltageDomainExists(s: S, domain_id: UInt32) -> bool;
pub open spec fn VoltageDomainSupportsMode(s: S, domain_id: UInt32, mode: UInt32) -> bool;
pub open spec fn IsRequestSupported(s: S, msg_id: UInt32) -> bool;
pub open spec fn AgentMaySetVoltageConfig(s: S, agent: AgentId, domain_id: UInt32) -> bool;
pub open spec fn CallingAgent(s: S) -> AgentId;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn VoltageDomain(s: S, domain_id: UInt32) -> VoltageDomainState;

} // verus!
