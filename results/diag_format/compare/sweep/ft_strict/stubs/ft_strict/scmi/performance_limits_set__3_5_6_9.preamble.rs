use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = (-4int) as i32;
pub spec const OUT_OF_RANGE: Int32 = (-6int) as i32;
pub spec const DENIED: Int32 = (-3int) as i32;
pub spec const result: Int32 = 1000;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn PerfDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsWithinDescribedLevels(s: S, domain_id: UInt32, level: UInt32) -> bool;

pub open spec fn CallerMayChangePerfLimits(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerfLimitMax(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PerfLimitMin(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PrevPerfLimitMax(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PrevPerfLimitMin(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn LimitFieldToLevel(s: S, domain_id: UInt32, field: UInt32) -> UInt32;

pub open spec fn PerfLevelEventuallyWithinLimits(s: S, domain_id: UInt32) -> bool;

} // verus!
