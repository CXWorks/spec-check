use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub agent_id: u32,
    pub identifier: u32,
    pub flags: u32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;
pub const DENIED: int32 = -3;
pub const NOT_FOUND: int32 = -4;
pub const IN_USE: int32 = -8;

pub open spec fn identifier(s: S) -> UInt32;

pub open spec fn flags(s: S) -> UInt32;

pub open spec fn IsPinOrGroupValid(s: S, id: UInt32) -> bool;

pub open spec fn AgentCanRequestPinOrGroup(s: S, id: UInt32) -> bool;

pub open spec fn IsPinOrGroupInUse(s: S, id: UInt32) -> bool;

pub open spec fn PinOrGroupControlledBy(old_s: S, id: UInt32, new_s: S) -> bool;

} // verus!
