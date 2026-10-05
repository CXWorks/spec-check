use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;
pub const OUT_OF_RANGE: Int32 = -6;

pub const domain_id: UInt32 = 1;
pub const range_max: UInt32 = 2;
pub const range_min: UInt32 = 3;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn PerfDomainExists(s: S, did: UInt32) -> bool;
pub open spec fn IsWithinDescribedLevels(s: S, did: UInt32, level: UInt32) -> bool;
pub open spec fn CallerMayChangePerfLimits(s: S, did: UInt32) -> bool;
pub open spec fn PerfLimitMax(s: S, did: UInt32) -> UInt32;
pub open spec fn PerfLimitMin(s: S, did: UInt32) -> UInt32;
pub open spec fn LimitFieldToLevel(s: S, did: UInt32, field: UInt32) -> UInt32;
pub open spec fn PrevPerfLimitMax(s: S, did: UInt32) -> UInt32;
pub open spec fn PrevPerfLimitMin(s: S, did: UInt32) -> UInt32;
pub open spec fn PerfLevelEventuallyWithinLimits(s: S, did: UInt32) -> bool;

} // verus!
