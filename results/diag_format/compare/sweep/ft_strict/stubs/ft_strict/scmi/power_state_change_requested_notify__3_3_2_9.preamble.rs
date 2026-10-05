use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn IsValidPowerDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> int;

pub open spec fn CallingAgent() -> AgentId;

pub open spec fn PowerStateChangeRequestedNotifyEnabled(s: S, agent: AgentId, domain_id: UInt32) -> bool;

} // verus!
