use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub msg_id: UInt32,
    pub agent_id: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn message_id(s: S) -> UInt32;

pub open spec fn IsValidMessageId(id: UInt32) -> bool;

pub open spec fn IsMessageImplemented(id: UInt32) -> bool;

pub open spec fn IsMessageAvailable(id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

} // verus!
