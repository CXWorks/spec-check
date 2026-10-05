use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -6;

pub const CLOCK_RATE_NOTIFY: UInt32 = 9;
pub const CLOCK_RATE_CHANGE_REQUESTED_NOTIFY: UInt32 = 10;

pub const attributes: UInt32 = 0;

pub open spec fn IsValidMessageId(message_id: UInt32) -> bool;

pub open spec fn IsMessageImplemented(message_id: UInt32) -> bool;

pub open spec fn IsNotificationAvailableToAgent(message_id: UInt32, agent: UInt32) -> bool;

pub open spec fn IsMessageAvailableToAgent(message_id: UInt32, agent: UInt32) -> bool;

pub open spec fn IsNotificationSupported(message_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

} // verus!
