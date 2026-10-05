use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub agent: AgentId,
    pub data: Seq<u32>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_FOUND: Int32 = -4;

pub spec const result: Int32 = (-100) as i32;

pub open spec fn IsValidPerfDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerfDomainSupportsLimitsNotify(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidNotifyEnable(s: S, notify_enable: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn LimitsNotifyEnabled(s: S, agent: AgentId, domain_id: UInt32) -> bool;

pub open spec fn CallingAgent(s: S) -> AgentId;

} // verus!
