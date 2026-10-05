use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub domain_id: UInt32,
    pub calling_agent: UInt32,
}

pub struct VoltageDomainInfo {
    pub mode: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NOT_FOUND: Int32 = -4;
pub const DENIED: Int32 = -3;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn calling_agent(s: S) -> UInt32;

pub open spec fn IsValidVoltageDomain(domain_id: UInt32) -> bool;

pub open spec fn IsRequestSupported() -> bool;

pub open spec fn AgentMayGetVoltageConfig(agent_id: UInt32, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn VoltageDomain(domain_id: UInt32) -> VoltageDomainInfo;

} // verus!
