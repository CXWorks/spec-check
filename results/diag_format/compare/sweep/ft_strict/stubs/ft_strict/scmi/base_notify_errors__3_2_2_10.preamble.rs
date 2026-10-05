use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub type AgentId = int;

pub const SUCCESS: Int32 = 0;
pub const INVALID_PARAMETERS: Int32 = -2;

pub open spec fn IsValidNotifyEnable(s: S, notify_enable: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> int;

pub open spec fn ErrorEventNotificationsEnabled(s: S, agent: AgentId) -> bool;

pub open spec fn CallingAgent() -> AgentId;

} // verus!
