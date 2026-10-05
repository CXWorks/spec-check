use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;

pub const agent_id: UInt32 = 1;

pub struct S {
    pub calling_agent: UInt32,
}

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidAgentId(id: UInt32) -> bool;

pub open spec fn CallingAgentId() -> UInt32;

pub open spec fn AgentName(id: UInt32) -> [UInt8; 16];

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 16], len: int) -> bool;

pub open spec fn NameStartsWithPlatform(name: [UInt8; 16]) -> bool;

} // verus!
