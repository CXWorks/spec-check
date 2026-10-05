use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -6;

pub open spec fn IsValidMessageId(message_id: UInt32) -> bool;
pub open spec fn IsMessageImplemented(message_id: UInt32) -> bool;
pub open spec fn IsPowerStateNotifyMessage(message_id: UInt32) -> bool;
pub open spec fn IsNotificationAvailableToAgent(message_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
