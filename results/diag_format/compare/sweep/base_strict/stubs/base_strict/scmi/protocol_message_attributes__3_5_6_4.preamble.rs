use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub message_id: UInt32,
    pub calling_agent: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -3;

pub const PERFORMANCE_NOTIFY_LIMITS: UInt32 = 10;
pub const PERFORMANCE_NOTIFY_LEVEL: UInt32 = 11;

pub open spec fn message_id(s: S) -> UInt32;
pub open spec fn calling_agent(s: S) -> UInt32;
pub open spec fn IsValidMessageId(message_id: UInt32) -> bool;
pub open spec fn IsMessageImplemented(message_id: UInt32) -> bool;
pub open spec fn IsNotificationImplemented(message_id: UInt32) -> bool;
pub open spec fn IsNotificationAvailableToAgent(message_id: UInt32, agent: UInt32) -> bool;
pub open spec fn HasDedicatedFastChannel(message_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn Bits(value: UInt32, high: int, low: int) -> UInt32;

} // verus!
