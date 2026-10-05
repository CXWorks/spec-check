use vstd::prelude::*;

verus! {

pub struct UInt32 {
    pub access_type: u32,
    pub value: u32,
}

pub type Int32 = i32;

pub struct S {
    pub state_id: u64,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

pub const result: Int32 = 100;

pub const BASE_SET_DEVICE_PERMISSIONS: u32 = 9;

pub open spec fn AgentExists(s: S, agent_id: UInt32) -> bool;

pub open spec fn DeviceExists(s: S, device_id: UInt32) -> bool;

pub open spec fn IsValidDevicePermissionFlags(s: S, flags: UInt32) -> bool;

pub open spec fn IsCommandSupported(s: S, command: u32) -> bool;

pub open spec fn CallerMaySetPermissionsOf(s: S, agent_id: UInt32) -> bool;

pub open spec fn DeviceAccessAllowed(s: S, agent_id: UInt32, device_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
