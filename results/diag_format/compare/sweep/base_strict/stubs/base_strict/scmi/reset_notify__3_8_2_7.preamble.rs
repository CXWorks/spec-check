use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type AgentId = u32;

pub struct S {
    pub domain_id_field: UInt32,
    pub notify_enable_field: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn notify_enable(s: S) -> UInt32;

pub open spec fn IsValidResetDomain(domain_id: UInt32) -> bool;

pub open spec fn IsValidNotifyEnable(notify_enable: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(value: UInt32, high: int, low: int) -> int;

pub open spec fn CallingAgent() -> AgentId;

pub open spec fn ResetNotificationsEnabled(agent: AgentId, domain_id: UInt32) -> bool;

} // verus!
