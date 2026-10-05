use vstd::prelude::*;

verus! {

pub type uint32 = Seq<int>;
pub type int32 = i32;
pub type AgentId = u32;
pub type CommandId = u32;

pub struct VoltageDomainState {
    pub voltage_level: int32,
}

pub struct S {
    pub domains: Map<uint32, VoltageDomainState>,
    pub queued: Seq<(CommandId, uint32, int32)>,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;
pub const DENIED: int32 = -3;
pub const NOT_FOUND: int32 = -4;

#[allow(non_upper_case_globals)]
pub const result: int32 = -100;

#[allow(non_upper_case_globals)]
pub const caller: AgentId = 7;

pub const VOLTAGE_LEVEL_SET: CommandId = 8;

pub open spec fn VoltageDomainExists(s: S, domain_id: uint32) -> bool;

pub open spec fn VoltageLevelSupported(s: S, domain_id: uint32, voltage_level: int32) -> bool;

pub open spec fn RequestSupported(s: S, domain_id: uint32, flags: uint32, voltage_level: int32) -> bool;

pub open spec fn AgentMaySetVoltageLevel(s: S, agent: AgentId, domain_id: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn VoltageDomain(s: S, domain_id: uint32) -> VoltageDomainState;

pub open spec fn CommandQueued(s: S, cmd: CommandId, domain_id: uint32, voltage_level: int32) -> bool;

} // verus!
