use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2) as i32;
pub spec const DENIED: Int32 = (-3) as i32;
pub spec const NOT_FOUND: Int32 = (-4) as i32;
pub spec const result: Int32 = (-100) as i32;

pub spec const caller: UInt32 = 0;

pub uninterp spec fn IsCommandImplemented(s: S, protocol_id: UInt32, message_id: UInt32) -> bool;

pub uninterp spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub uninterp spec fn AgentExists(s: S, agent_id: UInt32) -> bool;

pub uninterp spec fn DeviceExists(s: S, device_id: UInt32) -> bool;

pub uninterp spec fn IsValidDevicePermissionFlags(s: S, flags: UInt32) -> bool;

pub uninterp spec fn CallerMaySetAgentPermissions(s: S, caller_id: UInt32, agent_id: UInt32) -> bool;

pub uninterp spec fn Bits(x: UInt32, lo: int, hi: int) -> int;

pub uninterp spec fn AgentHasDeviceAccess(s: S, agent_id: UInt32, device_id: UInt32) -> bool;

} // verus!
