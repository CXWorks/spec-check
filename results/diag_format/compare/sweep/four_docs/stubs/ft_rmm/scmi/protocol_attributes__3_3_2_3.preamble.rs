use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: Int32 = 0;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn NumPowerDomains() -> UInt32;

pub open spec fn PlatformSupportsStatisticsRegion() -> bool;

pub open spec fn StatisticsRegionAddr(high: UInt32, low: UInt32) -> UInt64;

pub open spec fn StatisticsRegionBase() -> UInt64;

pub open spec fn IsInCallerMemoryMap(addr: UInt64) -> bool;

pub open spec fn StatisticsRegionSize() -> UInt32;

} // verus!
