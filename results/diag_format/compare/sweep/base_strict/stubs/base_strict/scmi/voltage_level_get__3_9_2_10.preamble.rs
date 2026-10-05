use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub num_voltage_domains: UInt32,
    pub num_agents: UInt32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const NOT_FOUND: Int32 = (-3int) as i32;
pub spec const DENIED: Int32 = (-4int) as i32;

pub spec const domain_id: UInt32 = 0;
pub spec const calling_agent: UInt32 = 1;

pub open spec fn IsValidVoltageDomain(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsRequestSupported(s: S, domain_id: UInt32) -> bool;
pub open spec fn AgentMayGetVoltageLevel(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn VoltageLevelDuringCommand(s: S, domain_id: UInt32) -> Int32;

} // verus!
