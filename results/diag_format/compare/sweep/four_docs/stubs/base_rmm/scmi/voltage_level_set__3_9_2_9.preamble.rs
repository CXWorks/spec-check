use vstd::prelude::*;

verus! {

pub type DomainId = u32;
pub type VoltageLevel = i32;
pub type AgentId = u32;
pub type MessageId = u32;

pub struct S {
    pub dummy: int,
}

pub struct VoltageDomainState {
    pub voltage_level: VoltageLevel,
}

pub spec const SUCCESS: int = 0;
pub spec const NOT_SUPPORTED: int = -1;
pub spec const INVALID_PARAMETERS: int = -2;
pub spec const DENIED: int = -3;
pub spec const NOT_FOUND: int = -4;

pub spec const VOLTAGE_LEVEL_SET: MessageId = 8;

pub uninterp spec fn domain_id_value() -> DomainId;
pub uninterp spec fn voltage_level_value() -> VoltageLevel;
pub uninterp spec fn flags_value() -> Seq<u32>;
pub uninterp spec fn caller_value() -> AgentId;

pub spec const domain_id: DomainId = domain_id_value();
pub spec const voltage_level: VoltageLevel = voltage_level_value();
pub spec const flags: Seq<u32> = flags_value();
pub spec const caller: AgentId = caller_value();

pub uninterp spec fn VoltageDomainExists(s: S, domain_id: DomainId) -> bool;
pub uninterp spec fn VoltageLevelSupported(s: S, domain_id: DomainId, voltage_level: VoltageLevel) -> bool;
pub uninterp spec fn RequestSupported(s: S, domain_id: DomainId, flags: Seq<u32>, voltage_level: VoltageLevel) -> bool;
pub uninterp spec fn AgentMaySetVoltageLevel(s: S, caller: AgentId, domain_id: DomainId) -> bool;
pub uninterp spec fn ResultEqual(result: int, code: int) -> bool;
pub uninterp spec fn VoltageDomain(s: S, domain_id: DomainId) -> VoltageDomainState;
pub uninterp spec fn CommandQueued(s: S, message_id: MessageId, domain_id: DomainId, voltage_level: VoltageLevel) -> bool;

} // verus!
