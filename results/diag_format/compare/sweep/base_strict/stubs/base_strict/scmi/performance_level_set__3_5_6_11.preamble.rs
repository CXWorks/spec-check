use vstd::prelude::*;

verus! {

pub type UInt64 = u64;
pub type RmiStatusCode = u64;
pub type DomainId = u64;
pub type AgentId = u64;
pub type PerformanceLevel = u64;

pub spec const NOT_FOUND: RmiStatusCode = 1;
pub spec const OUT_OF_RANGE: RmiStatusCode = 2;
pub spec const DENIED: RmiStatusCode = 3;

pub spec const domain_id: DomainId = 10;
pub spec const performance_level: PerformanceLevel = 20;
pub spec const calling_agent: AgentId = 30;

pub struct PerformanceDomainState {
    pub requested_level: PerformanceLevel,
}

pub struct S {
    pub domains: Map<DomainId, PerformanceDomainState>,
}

pub open spec fn IsValidPerformanceDomain(s: S, d: DomainId) -> bool;

pub open spec fn IsWithinAllowedPerformanceRange(s: S, d: DomainId, level: PerformanceLevel) -> bool;

pub open spec fn AgentPermittedToSetPerformanceLevel(s: S, agent: AgentId, d: DomainId) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn PerformanceLevelRequestScheduled(s: S, d: DomainId, level: PerformanceLevel) -> bool;

pub open spec fn LevelIndexingModeInUse(s: S, d: DomainId) -> bool;

pub open spec fn PlatformPolicyDeterminesPerformanceLevel(s: S, d: DomainId) -> bool;

pub open spec fn PerformanceDomain(s: S, d: DomainId) -> PerformanceDomainState;

} // verus!
