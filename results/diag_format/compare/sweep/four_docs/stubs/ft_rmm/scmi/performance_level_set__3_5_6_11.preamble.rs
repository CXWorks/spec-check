use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct PerformanceDomainState {
    pub performance_level: UInt32,
}

pub struct S {
    pub domains: Map<UInt32, PerformanceDomainState>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;
pub const OUT_OF_RANGE: Int32 = -6;
pub const DENIED: Int32 = -3;

#[allow(non_upper_case_globals)]
pub const caller: AgentId = 7;

#[allow(non_upper_case_globals)]
pub const result: Int32 = -100;

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsInAllowedPerformanceRange(s: S, domain_id: UInt32, performance_level: UInt32) -> bool;

pub open spec fn AgentMaySetPerformanceLevel(s: S, agent: AgentId, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PlatformHasAcceptedAndScheduled(s: S, domain_id: UInt32, performance_level: UInt32) -> bool;

pub open spec fn PerformanceDomain(s: S, domain_id: UInt32) -> PerformanceDomainState;

} // verus!
