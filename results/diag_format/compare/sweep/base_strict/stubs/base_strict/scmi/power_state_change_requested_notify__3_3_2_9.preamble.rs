use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type AgentId = u32;
pub type DomainId = u32;

pub struct S {
    pub domain_id: DomainId,
    pub notify_enable: UInt64,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn domain_id(s: S) -> DomainId;

pub open spec fn notify_enable(s: S) -> UInt64;

pub open spec fn IsValidPowerDomain(domain: DomainId) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits64(value: UInt64, hi: int, lo: int) -> UInt64;

pub open spec fn CallingAgent() -> AgentId;

pub open spec fn PowerStateChangeRequestedNotifyEnabled(agent: AgentId, domain: DomainId) -> bool;

} // verus!
