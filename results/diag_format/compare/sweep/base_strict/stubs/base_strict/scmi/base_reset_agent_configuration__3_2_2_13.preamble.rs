use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -7;

pub const BASE_RESET_AGENT_CONFIGURATION: UInt32 = 0x17;

pub const agent_id: UInt32 = 1;
pub const caller_agent_id: UInt32 = 2;
pub const flags: UInt32 = 3;

pub open spec fn IsCommandSupported(cmd: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub open spec fn AgentExists(s: S, id: UInt32) -> bool;

pub open spec fn IsValidResetAgentFlags(s: S, f: UInt32) -> bool;

pub open spec fn CallerMayResetAgentConfiguration(s: S, caller: UInt32, id: UInt32) -> bool;

pub open spec fn DedicatedResourcesInDefaultState(s: S, id: UInt32) -> bool;

pub open spec fn SharedResourcesMeetRemainingAgentsRequirements(s: S, id: UInt32) -> bool;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> int;

pub open spec fn AgentPermissionsAreImplementationDefinedDefaults(s: S, id: UInt32) -> bool;

pub open spec fn AgentPermissionsUnchanged(s: S, id: UInt32) -> bool;

} // verus!
