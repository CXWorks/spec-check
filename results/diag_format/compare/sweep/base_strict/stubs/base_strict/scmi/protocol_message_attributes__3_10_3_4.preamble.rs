use vstd::prelude::*;

verus! {

pub type uint32 = u32;

pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -3;
pub const NOT_FOUND: i32 = -4;

pub const POWERCAP_CAP_NOTIFY: uint32 = 0x0;
pub const POWERCAP_MEASUREMENTS_NOTIFY: uint32 = 0x1;

#[allow(non_upper_case_globals)]
pub const calling_agent: AgentId = 0;

pub open spec fn ResultEqual(status: int, code: i32) -> bool;

pub open spec fn IsValidMessage(message_id: uint32) -> bool;

pub open spec fn IsImplementedMessage(message_id: uint32) -> bool;

pub open spec fn NotificationsImplemented(message_id: uint32) -> bool;

pub open spec fn NotificationsAvailableToAgent(message_id: uint32, agent: AgentId) -> bool;

pub open spec fn IsAvailableToAgent(message_id: uint32, agent: AgentId) -> bool;

pub open spec fn NotificationsSupported(message_id: uint32, agent: AgentId) -> bool;

pub open spec fn HasDedicatedFastChannel(message_id: uint32) -> bool;

pub open spec fn Bits(value: uint32, high: int, low: int) -> int;

} // verus!
