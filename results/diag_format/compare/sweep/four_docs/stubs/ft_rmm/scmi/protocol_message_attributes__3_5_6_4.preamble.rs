use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub num_agents: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub const calling_agent: AgentId = 0;
pub const result: Int32 = 1;

pub open spec fn IsValidMessage(s: S, message_id: UInt32) -> bool;
pub open spec fn IsImplementedMessage(s: S, message_id: UInt32) -> bool;
pub open spec fn IsPerformanceNotifyMessage(s: S, message_id: UInt32) -> bool;
pub open spec fn NotificationsImplemented(s: S, message_id: UInt32) -> bool;
pub open spec fn NotificationsAvailableToAgent(s: S, message_id: UInt32, agent: AgentId) -> bool;
pub open spec fn HasDedicatedFastChannel(s: S, message_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
