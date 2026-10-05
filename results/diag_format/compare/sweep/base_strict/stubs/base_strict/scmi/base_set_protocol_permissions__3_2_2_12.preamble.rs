use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

pub const BASE_SET_PROTOCOL_PERMISSIONS: UInt32 = 0xA;

pub const agent_id: UInt32 = 101;
pub const device_id: UInt32 = 102;
pub const command_id: UInt32 = 103;
pub const flags: UInt32 = 104;
pub const caller: UInt32 = 105;

pub open spec fn IsCommandSupported(cmd: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub open spec fn AgentExists(s: S, agent: UInt32) -> bool;

pub open spec fn DeviceExists(s: S, device: UInt32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn ProtocolExists(s: S, protocol: UInt32) -> bool;

pub open spec fn IsValidFlags(s: S, f: UInt32) -> bool;

pub open spec fn CallerMaySetProtocolPermissions(s: S, c: UInt32, agent: UInt32) -> bool;

pub open spec fn ProtocolAccessAllowed(s: S, agent: UInt32, device: UInt32, protocol: UInt32) -> bool;

pub open spec fn DeviceAccessAllowed(s: S, agent: UInt32, device: UInt32) -> bool;

} // verus!
