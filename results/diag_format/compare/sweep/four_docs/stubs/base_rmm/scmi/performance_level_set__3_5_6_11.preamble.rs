use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type DomainId = u32;
pub type PerformanceLevel = u32;
pub type AgentId = u32;

pub struct PerformanceDomainState {
    pub performance_level: PerformanceLevel,
}

pub struct S {
    pub dummy: u32,
}

impl S {
    pub open spec fn PerformanceDomain(self, id: DomainId) -> PerformanceDomainState;
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = (-1) as i32;
pub spec const OUT_OF_RANGE: Int32 = (-2) as i32;
pub spec const DENIED: Int32 = (-3) as i32;

pub spec const domain_id: DomainId = 1;
pub spec const performance_level: PerformanceLevel = 2;
pub spec const caller: AgentId = 3;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn IsValidPerformanceDomain(s: S, domain_id: DomainId) -> bool;
pub open spec fn IsInAllowedPerformanceRange(s: S, domain_id: DomainId, performance_level: PerformanceLevel) -> bool;
pub open spec fn AgentMaySetPerformanceLevel(s: S, caller: AgentId, domain_id: DomainId) -> bool;
pub open spec fn PlatformHasAcceptedAndScheduled(s: S, domain_id: DomainId, performance_level: PerformanceLevel) -> bool;

} // verus!
