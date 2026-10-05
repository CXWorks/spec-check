use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub agent_id: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn IsValidMessageId(s: S, message_id: UInt32) -> bool;
pub open spec fn IsMessageImplemented(s: S, message_id: UInt32) -> bool;
pub open spec fn IsPowerStateNotifyMessage(s: S, message_id: UInt32) -> bool;
pub open spec fn IsNotificationAvailableToAgent(s: S, message_id: UInt32) -> bool;
pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

} // verus!
