use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const result: Int32 = 7;

pub open spec fn IsValidResetDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidNotifyEnable(s: S, notify_enable: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn ResetNotificationsEnabled(s: S, agent: AgentId, domain_id: UInt32) -> bool;

pub open spec fn CallingAgent() -> AgentId;

} // verus!
