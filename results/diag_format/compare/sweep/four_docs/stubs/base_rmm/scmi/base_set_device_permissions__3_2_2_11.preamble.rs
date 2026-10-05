use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct DevicePermissionFlags {
    pub access_type: u32,
}

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = 1;
pub spec const INVALID_PARAMETERS: Int32 = 2;
pub spec const NOT_SUPPORTED: Int32 = 3;
pub spec const DENIED: Int32 = 4;

pub spec const BASE_SET_DEVICE_PERMISSIONS: UInt32 = 11;

pub open spec fn agent_id(s: S) -> UInt64;

pub open spec fn device_id(s: S) -> UInt64;

pub open spec fn flags(s: S) -> DevicePermissionFlags;

pub open spec fn AgentExists(s: S, agent: UInt64) -> bool;

pub open spec fn DeviceExists(s: S, device: UInt64) -> bool;

pub open spec fn IsValidDevicePermissionFlags(s: S, f: DevicePermissionFlags) -> bool;

pub open spec fn IsCommandSupported(s: S, cmd: UInt32) -> bool;

pub open spec fn CallerMaySetPermissionsOf(s: S, agent: UInt64) -> bool;

pub open spec fn DeviceAccessAllowed(s: S, agent: UInt64, device: UInt64) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
