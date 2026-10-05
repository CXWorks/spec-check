use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0i32;
pub spec const INVALID_PARAMETERS: Int32 = -2i32;
pub spec const NOT_FOUND: Int32 = -4i32;
pub spec const result: Int32 = -100i32;

pub uninterp spec fn IsValidPowerDomain(s: S, domain_id: UInt32) -> bool;
pub uninterp spec fn IsValidNotifyEnable(s: S, notify_enable: UInt32) -> bool;
pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub uninterp spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub uninterp spec fn CallingAgent() -> AgentId;
pub uninterp spec fn PowerStateChangedNotifyEnabled(s: S, agent: AgentId, domain_id: UInt32) -> bool;

} // verus!
