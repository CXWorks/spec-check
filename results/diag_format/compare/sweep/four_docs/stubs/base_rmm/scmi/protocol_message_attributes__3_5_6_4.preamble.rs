use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type MessageId = u32;
pub type AgentId = u32;

pub struct S {
    pub message_id: MessageId,
    pub calling_agent: AgentId,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn message_id(s: S) -> MessageId;

pub open spec fn calling_agent(s: S) -> AgentId;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidMessage(s: S, msg: MessageId) -> bool;

pub open spec fn IsImplementedMessage(s: S, msg: MessageId) -> bool;

pub open spec fn IsPerformanceNotifyMessage(s: S, msg: MessageId) -> bool;

pub open spec fn NotificationsImplemented(s: S, msg: MessageId) -> bool;

pub open spec fn NotificationsAvailableToAgent(s: S, msg: MessageId, agent: AgentId) -> bool;

pub open spec fn HasDedicatedFastChannel(s: S, msg: MessageId) -> bool;

} // verus!
