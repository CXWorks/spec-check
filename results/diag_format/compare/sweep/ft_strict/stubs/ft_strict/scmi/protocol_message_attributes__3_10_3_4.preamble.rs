use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;

pub struct AgentId {
    pub id: u32,
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -4;
pub const result: int32 = 7;

pub const POWERCAP_CAP_NOTIFY: uint32 = 10;
pub const POWERCAP_MEASUREMENTS_NOTIFY: uint32 = 11;

pub const calling_agent: AgentId = AgentId { id: 0 };

pub open spec fn IsValidMessage(s: S, message_id: uint32) -> bool;
pub open spec fn IsImplementedMessage(s: S, message_id: uint32) -> bool;
pub open spec fn ResultEqual(a: int32, b: int32) -> bool;
pub open spec fn NotificationsImplemented(s: S, message_id: uint32) -> bool;
pub open spec fn NotificationsAvailableToAgent(s: S, message_id: uint32, agent: AgentId) -> bool;
pub open spec fn IsAvailableToAgent(s: S, message_id: uint32, agent: AgentId) -> bool;
pub open spec fn NotificationsSupported(s: S, message_id: uint32, agent: AgentId) -> bool;
pub open spec fn Bits(value: uint32, hi: int, lo: int) -> int;
pub open spec fn HasDedicatedFastChannel(s: S, message_id: uint32) -> bool;

} // verus!
