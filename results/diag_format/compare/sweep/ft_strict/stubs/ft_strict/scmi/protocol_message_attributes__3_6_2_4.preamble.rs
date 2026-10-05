use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct CallingAgent {
    pub id: u32,
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -3;

pub const CLOCK_RATE_NOTIFY: UInt32 = 0xB;
pub const CLOCK_RATE_CHANGE_REQUESTED_NOTIFY: UInt32 = 0xC;

pub open spec fn IsValidMessageId(s: S, message_id: UInt32) -> bool;

pub open spec fn IsMessageImplemented(s: S, message_id: UInt32) -> bool;

pub open spec fn IsNotificationAvailableToAgent(s: S, message_id: UInt32, calling_agent: CallingAgent) -> bool;

pub open spec fn IsMessageAvailableToAgent(s: S, message_id: UInt32, calling_agent: CallingAgent) -> bool;

pub open spec fn IsNotificationSupported(s: S, message_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

} // verus!
