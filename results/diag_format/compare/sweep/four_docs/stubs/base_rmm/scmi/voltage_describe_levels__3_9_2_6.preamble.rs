use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub placeholder: int,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_SUPPORTED: int32 = -1;
pub spec const DENIED: int32 = -3;
pub spec const NOT_FOUND: int32 = -4;
pub spec const OUT_OF_RANGE: int32 = -5;

pub spec const VOLTAGE_DESCRIBE_LEVELS: uint32 = 4;

pub spec const calling_agent: uint32 = 0;

pub open spec fn VoltageDomainExists(domain_id: uint32) -> bool;

pub open spec fn ResultEqual(status: int32, expected: int32) -> bool;

pub open spec fn IsValidVoltageLevelIndex(domain_id: uint32, level_index: uint32) -> bool;

pub open spec fn IsRequestSupported(message_id: uint32) -> bool;

pub open spec fn AgentMayGetVoltageLevels(agent_id: uint32, domain_id: uint32) -> bool;

pub open spec fn LowestVoltageLevel(domain_id: uint32) -> int32;

pub open spec fn HighestVoltageLevel(domain_id: uint32) -> int32;

pub open spec fn VoltageStepSize(domain_id: uint32) -> int32;

pub open spec fn IsSupportedVoltageLevel(domain_id: uint32, level: int32) -> bool;

} // verus!
