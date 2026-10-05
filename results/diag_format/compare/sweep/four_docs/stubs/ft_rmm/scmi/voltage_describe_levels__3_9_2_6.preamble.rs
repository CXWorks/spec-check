use vstd::prelude::*;

verus! {

pub type uint32 = u32;
pub type int32 = i32;
pub type AgentId = u32;
pub type MessageId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const OUT_OF_RANGE: int32 = -3;
pub const DENIED: int32 = -4;
pub const NOT_FOUND: int32 = -5;

pub const VOLTAGE_DESCRIBE_LEVELS: MessageId = 4;

pub const calling_agent: AgentId = 0;

pub const result: int32 = 0;

pub open spec fn VoltageDomainExists(s: S, domain_id: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn IsValidVoltageLevelIndex(s: S, domain_id: uint32, level_index: uint32) -> bool;

pub open spec fn IsRequestSupported(s: S, msg: MessageId) -> bool;

pub open spec fn AgentMayGetVoltageLevels(s: S, agent: AgentId, domain_id: uint32) -> bool;

pub open spec fn LowestVoltageLevel(s: S, domain_id: uint32) -> int32;

pub open spec fn HighestVoltageLevel(s: S, domain_id: uint32) -> int32;

pub open spec fn VoltageStepSize(s: S, domain_id: uint32) -> int32;

pub open spec fn IsSupportedVoltageLevel(s: S, domain_id: uint32, voltage: int32) -> bool;

} // verus!
