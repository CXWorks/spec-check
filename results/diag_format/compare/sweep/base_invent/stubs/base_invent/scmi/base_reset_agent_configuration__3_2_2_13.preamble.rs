use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub num_agents: nat,
    pub supported_commands: Set<u32>,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;
pub const DENIED: int32 = -3;
pub const NOT_FOUND: int32 = -4;

#[allow(non_upper_case_globals)]
pub const agent_id: uint32 = 1;
#[allow(non_upper_case_globals)]
pub const caller_id: uint32 = 2;
#[allow(non_upper_case_globals)]
pub const flags: uint32 = 3;

pub open spec fn AgentExists(s: S, agent: uint32) -> bool;

pub open spec fn CommandSupported(s: S, command: uint32) -> bool;

pub open spec fn AgentCanReset(s: S, caller: uint32, agent: uint32) -> bool;

pub open spec fn AgentPermissionsReset(old_s: S, agent: uint32, new_s: S) -> bool;

pub open spec fn PlatformResourcesReset(old_s: S, agent: uint32, new_s: S) -> bool;

} // verus!
