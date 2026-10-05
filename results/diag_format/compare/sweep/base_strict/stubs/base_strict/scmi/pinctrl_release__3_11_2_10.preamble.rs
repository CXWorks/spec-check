use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type UInt64 = u64;
pub type UInt32 = u32;
pub type AgentId = u32;

pub struct S {
    pub cmd_input_flags: UInt64,
    pub cmd_input_identifier: UInt32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -4;
pub const INVALID_PARAMETERS: int32 = -2;

pub open spec fn Bits64(x: u64, hi: u64, lo: u64) -> u64;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn IsValidPinOrGroup(identifier: UInt32, selector: u64) -> bool;

pub open spec fn CallingAgent() -> AgentId;

pub open spec fn HasExclusiveControl(agent: AgentId, identifier: UInt32, selector: u64) -> bool;

} // verus!
