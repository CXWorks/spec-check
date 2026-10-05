use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub agent_id_field: UInt64,
    pub device_id_field: UInt64,
    pub flags_field: UInt64,
    pub caller_field: UInt64,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2) as i32;
pub spec const DENIED: Int32 = (-3) as i32;
pub spec const NOT_FOUND: Int32 = (-4) as i32;

pub open spec fn IsCommandImplemented(s: S, interface_id: int, command_id: int) -> bool;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub open spec fn AgentExists(s: S, agent: UInt64) -> bool;

pub open spec fn DeviceExists(s: S, device: UInt64) -> bool;

pub open spec fn IsValidDevicePermissionFlags(s: S, f: UInt64) -> bool;

pub open spec fn CallerMaySetAgentPermissions(s: S, c: UInt64, agent: UInt64) -> bool;

pub open spec fn AgentHasDeviceAccess(s: S, agent: UInt64, device: UInt64) -> bool;

pub open spec fn Bits64(value: UInt64, hi: int, lo: int) -> UInt64;

pub open spec fn agent_id(s: S) -> UInt64;

pub open spec fn device_id(s: S) -> UInt64;

pub open spec fn flags(s: S) -> UInt64;

pub open spec fn caller(s: S) -> UInt64;

} // verus!
