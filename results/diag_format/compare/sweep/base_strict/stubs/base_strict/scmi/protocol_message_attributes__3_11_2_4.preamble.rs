use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -3;

pub open spec fn IsMessageValid(protocol_id: UInt32, message_id: UInt32) -> bool;
pub open spec fn IsMessageImplemented(protocol_id: UInt32, message_id: UInt32) -> bool;
pub open spec fn IsMessageAvailableToCallingAgent(protocol_id: UInt32, message_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
