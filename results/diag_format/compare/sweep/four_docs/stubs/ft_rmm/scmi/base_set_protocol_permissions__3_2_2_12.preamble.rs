use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub agents: Set<UInt32>,
    pub devices: Set<UInt32>,
    pub protocols: Set<int>,
    pub implemented_commands: Set<UInt32>,
    pub protocol_permissions: Map<(UInt32, UInt32, int), Int32>,
    pub device_permissions: Map<(UInt32, UInt32), Int32>,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const DENIED: Int32 = -3;
pub spec const NOT_FOUND: Int32 = -4;
pub spec const ALLOWED: Int32 = 1;

pub spec const BASE_SET_PROTOCOL_PERMISSIONS: UInt32 = 11;

pub spec const caller: UInt32 = 0;
pub spec const result: Int32 = 7;

pub open spec fn IsCommandImplemented(s: S, cmd: UInt32) -> bool;
pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;
pub open spec fn AgentExists(s: S, agent_id: UInt32) -> bool;
pub open spec fn DeviceExists(s: S, device_id: UInt32) -> bool;
pub open spec fn ProtocolExists(s: S, protocol_id: int) -> bool;
pub open spec fn IsValidFlags(s: S, flags: UInt32) -> bool;
pub open spec fn CallerMaySetProtocolPermissions(s: S, caller_id: UInt32, agent_id: UInt32) -> bool;
pub open spec fn ProtocolPermission(s: S, agent_id: UInt32, device_id: UInt32, protocol_id: int) -> Int32;
pub open spec fn DeviceAccessPermission(s: S, agent_id: UInt32, device_id: UInt32) -> Int32;

} // verus!
