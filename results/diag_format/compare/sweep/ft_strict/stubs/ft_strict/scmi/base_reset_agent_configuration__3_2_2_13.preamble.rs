use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

pub const result: Int32 = 7;

pub const caller_agent_id: UInt32 = 0;

pub const BASE_RESET_AGENT_CONFIGURATION: UInt32 = 0xB;

pub open spec fn IsCommandSupported(s: S, cmd: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn AgentExists(s: S, agent_id: UInt32) -> bool;

pub open spec fn IsValidResetAgentFlags(s: S, flags: UInt32) -> bool;

pub open spec fn CallerMayResetAgentConfiguration(s: S, caller: UInt32, agent_id: UInt32) -> bool;

pub open spec fn DedicatedResourcesInDefaultState(s: S, agent_id: UInt32) -> bool;

pub open spec fn SharedResourcesMeetRemainingAgentsRequirements(s: S, agent_id: UInt32) -> bool;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn AgentPermissionsAreImplementationDefinedDefaults(s: S, agent_id: UInt32) -> bool;

pub open spec fn AgentPermissionsUnchanged(s: S, agent_id: UInt32) -> bool;

} // verus!
