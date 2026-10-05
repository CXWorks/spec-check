use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub const SUCCESS: Int32 = 0;

pub enum RmiStatusCode {
    Success,
    ErrorInput,
    ErrorRealm,
    ErrorRec,
    ErrorRtt,
}

pub struct S {
    pub dummy: int,
}

pub trait BitSliceIndex {
    spec fn spec_index(self, r: core::ops::Range<int>) -> int;
}

pub uninterp spec fn u32_bits(x: u32, hi: int, lo: int) -> int;

impl BitSliceIndex for u32 {
    open spec fn spec_index(self, r: core::ops::Range<int>) -> int {
        u32_bits(self, r.start, r.end)
    }
}

pub uninterp spec fn NumPowerDomains() -> int;

pub uninterp spec fn PlatformSupportsStatisticsRegion() -> bool;

pub uninterp spec fn StatisticsRegionAddr(high: UInt32, low: UInt32) -> UInt64;

pub uninterp spec fn StatisticsRegionBase() -> UInt64;

pub uninterp spec fn IsInCallerMemoryMap(addr: UInt64) -> bool;

pub uninterp spec fn StatisticsRegionSize() -> UInt32;

} // verus!
