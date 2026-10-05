use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -3;
pub const NOT_FOUND: i32 = -4;

pub open spec fn BaseSetDevicePermissionsSupported(s: S) -> bool;

pub open spec fn AgentExists(s: S, agent_id: u32) -> bool;

pub open spec fn DeviceExists(s: S, device_id: u32) -> bool;

pub open spec fn CallerAllowedToSetAgentPermissions(s: S, agent_id: u32) -> bool;

pub open spec fn AgentDeviceAccessAllowed(s: S, agent_id: u32, device_id: u32) -> bool;

pub open spec fn DevicePermissionsUnchangedExcept(old_s: S, new_s: S, agent_id: u32, device_id: u32) -> bool;

} // verus!
