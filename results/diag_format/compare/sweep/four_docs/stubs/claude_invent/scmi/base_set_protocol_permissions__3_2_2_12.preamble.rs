use vstd::prelude::*;
verus! {

pub type ScmiStatusCode = i32;

pub const SUCCESS: ScmiStatusCode = 0;
pub const NOT_SUPPORTED: ScmiStatusCode = -1;
pub const INVALID_PARAMETERS: ScmiStatusCode = -2;
pub const DENIED: ScmiStatusCode = -3;
pub const NOT_FOUND: ScmiStatusCode = -4;

pub struct S {
    pub dummy: int,
}

pub open spec fn BaseSetProtocolPermissionsSupported(s: S) -> bool;

pub open spec fn AgentExists(s: S, agent_id: u32) -> bool;

pub open spec fn DeviceExists(s: S, device_id: u32) -> bool;

pub open spec fn ProtocolExists(s: S, protocol_id: u32) -> bool;

pub open spec fn CallerMaySetProtocolPermissions(s: S, agent_id: u32) -> bool;

pub open spec fn AgentProtocolAccessAllowed(s: S, agent_id: u32, device_id: u32, protocol_id: u32) -> bool;

pub open spec fn AgentDeviceAccessAllowed(s: S, agent_id: u32, device_id: u32) -> bool;

} // verus!
