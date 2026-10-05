use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type AgentId = u32;

pub struct S {
    pub num_perf_domains: u32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as Int32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as Int32;
pub spec const NOT_FOUND: Int32 = (-4int) as Int32;

#[allow(non_upper_case_globals)]
pub spec const domain_id: UInt32 = 0x1000;
#[allow(non_upper_case_globals)]
pub spec const notify_enable: UInt32 = 0x2001;

pub uninterp spec fn IsValidPerfDomain(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn PerfDomainSupportsLimitsNotify(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn IsValidNotifyEnable(s: S, notify_enable: UInt32) -> bool;

pub uninterp spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub uninterp spec fn Bits(value: UInt32, high: int, low: int) -> UInt32;

pub uninterp spec fn CallingAgent() -> AgentId;

pub uninterp spec fn LimitsNotifyEnabled(agent: AgentId, domain_id: UInt32) -> bool;

} // verus!
