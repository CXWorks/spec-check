use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub num_messages: u32,
}

pub spec const SUCCESS: Int32 = 0i32;
pub spec const NOT_FOUND: Int32 = -4i32;
pub spec const result: Int32 = 1i32;

pub open spec fn IsValidMessageId(s: S, message_id: UInt32) -> bool;
pub open spec fn IsMessageImplemented(s: S, message_id: UInt32) -> bool;
pub open spec fn IsMessageAvailableToAgent(s: S, message_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
