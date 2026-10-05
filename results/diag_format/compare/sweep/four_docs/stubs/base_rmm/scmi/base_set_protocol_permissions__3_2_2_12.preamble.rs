use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const DENIED: Int32 = -3;
pub spec const NOT_FOUND: Int32 = -4;
pub spec const ALLOWED: Int32 = 1;

pub spec const BASE_SET_PROTOCOL_PERMISSIONS: UInt32 = 9;

pub spec const agent_id: UInt32 = 0;
pub spec const device_id: UInt32 = 0;
pub spec const command_id: UInt32 = 0;
pub spec const caller: UInt32 = 0;
pub spec const flags: Seq<UInt32> = Seq::empty();

pub open spec fn IsCommandImplemented(cmd: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn AgentExists(s: S, agent: UInt32) -> bool;
pub open spec fn DeviceExists(s: S, device: UInt32) -> bool;
pub open spec fn ProtocolExists(s: S, protocol: UInt32) -> bool;
pub open spec fn IsValidFlags(s: S, f: Seq<UInt32>) -> bool;
pub open spec fn CallerMaySetProtocolPermissions(s: S, c: UInt32, agent: UInt32) -> bool;
pub open spec fn ProtocolPermission(s: S, agent: UInt32, device: UInt32, protocol: UInt32) -> Int32;
pub open spec fn DeviceAccessPermission(s: S, agent: UInt32, device: UInt32) -> Int32;

} // verus!
