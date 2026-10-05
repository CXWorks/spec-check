use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -3;

pub const PERFORMANCE_NOTIFY_LEVEL: UInt32 = 10;
pub const PERFORMANCE_NOTIFY_LIMITS: UInt32 = 9;

pub const calling_agent: AgentId = 0;

pub open spec fn IsValidMessageId(s: S, message_id: UInt32) -> bool;

pub open spec fn IsMessageImplemented(s: S, message_id: UInt32) -> bool;

pub open spec fn IsNotificationImplemented(s: S, message_id: UInt32) -> bool;

pub open spec fn IsNotificationAvailableToAgent(s: S, message_id: UInt32, agent: AgentId) -> bool;

pub open spec fn HasDedicatedFastChannel(s: S, message_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn Bits(value: UInt32, high: int, low: int) -> int;

} // verus!
