use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0i32;
pub const NOT_SUPPORTED: i32 = -1i32;
pub const NOT_FOUND: i32 = -2i32;
pub const INVALID_PARAMETERS: i32 = -3i32;
pub const DENIED: i32 = -4i32;

pub open spec fn BaseResetAgentConfigurationSupported(s: S) -> bool;

pub open spec fn AgentExists(s: S, agent_id: u32) -> bool;

pub open spec fn AgentMayResetAgentConfiguration(s: S, caller_agent_id: u32, agent_id: u32) -> bool;

pub open spec fn AgentPlatformResourceSettingsReset(old_s: S, new_s: S, agent_id: u32) -> bool;

pub open spec fn SharedPlatformResourcesMeetRemainingAgentsRequirements(old_s: S, new_s: S, agent_id: u32) -> bool;

pub open spec fn AgentAccessPermissionsResetToDefault(old_s: S, new_s: S, agent_id: u32) -> bool;

pub open spec fn AgentAccessPermissionsUnchanged(old_s: S, new_s: S, agent_id: u32) -> bool;

} // verus!
