use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;
pub type Result = i32;

pub const SUCCESS: Result = 0;
pub const NOT_FOUND: Result = 1;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsValidAgentId(agent_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, expected: Result) -> bool;

pub open spec fn CallingAgentId() -> UInt32;

pub open spec fn AgentName(s: S, agent_id: UInt32) -> [UInt8; 16];

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 16], len: int) -> bool;

pub open spec fn NameStartsWithPlatform(name: [UInt8; 16]) -> bool;

} // verus!
