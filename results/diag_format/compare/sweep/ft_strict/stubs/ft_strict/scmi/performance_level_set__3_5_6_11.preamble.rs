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
pub const OUT_OF_RANGE: Int32 = -2;
pub const DENIED: Int32 = -3;

#[allow(non_upper_case_globals)]
pub const calling_agent: AgentId = 0;

#[allow(non_upper_case_globals)]
pub const result: Int32 = 100;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsWithinAllowedPerformanceRange(s: S, domain_id: UInt32, performance_level: UInt32) -> bool;

pub open spec fn AgentPermittedToSetPerformanceLevel(s: S, agent: AgentId, domain_id: UInt32) -> bool;

pub open spec fn PerformanceLevelRequestScheduled(s: S, domain_id: UInt32, performance_level: UInt32) -> bool;

pub open spec fn LevelIndexingModeInUse(s: S, domain_id: UInt32) -> bool;

pub open spec fn PlatformPolicyDeterminesPerformanceLevel(s: S, domain_id: UInt32) -> bool;

} // verus!
