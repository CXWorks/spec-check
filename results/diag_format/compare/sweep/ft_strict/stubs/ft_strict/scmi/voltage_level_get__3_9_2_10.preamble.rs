use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub num_domains: nat,
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const DENIED: Int32 = -3;
pub spec const NOT_FOUND: Int32 = -4;

pub spec const calling_agent: AgentId = 0;

pub open spec fn IsValidVoltageDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsRequestSupported(s: S, domain_id: UInt32) -> bool;

pub open spec fn AgentMayGetVoltageLevel(s: S, agent: AgentId, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: (Int32, Int32), b: (Int32, Int32)) -> bool;

pub open spec fn VoltageLevelDuringCommand(s: S, domain_id: UInt32) -> Int32;

} // verus!
