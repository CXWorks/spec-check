use vstd::prelude::*;

verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;

pub open spec fn IsValidAgentId(agent_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn CallingAgentId() -> UInt32;

pub open spec fn AgentName(agent_id: UInt32) -> [UInt8; 16];

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 16]) -> bool;

pub open spec fn StringLength(name: [UInt8; 16]) -> int;

pub open spec fn StringStartsWith(name: [UInt8; 16], prefix: &str) -> bool;

} // verus!
