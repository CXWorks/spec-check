use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub const SUCCESS: Int32 = 0;

pub const NOT_FOUND: Int32 = -1;

pub const DENIED: Int32 = -2;

pub const OUT_OF_RANGE: Int32 = -3;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn AgentPermittedToSetPerformanceLevel(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerformanceLevelInAllowedRange(s: S, domain_id: UInt32, performance_level: UInt32) -> bool;

pub open spec fn PerformanceLevelRequestScheduled(s: S, domain_id: UInt32, performance_level: UInt32) -> bool;

} // verus!
