use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub state_id: int,
}

pub open spec fn bit_range_u32(x: u32, r: core::ops::Range<int>) -> int;

pub trait BitSliceIndex {
    spec fn spec_index(self, r: core::ops::Range<int>) -> int;
}

impl BitSliceIndex for u32 {
    open spec fn spec_index(self, r: core::ops::Range<int>) -> int {
        bit_range_u32(self, r)
    }
}

pub open spec fn NumPerformanceDomains() -> int;

pub open spec fn IsInCallerMemoryMap(address_high: uint32, address_low: uint32) -> bool;

pub open spec fn IsAligned64(address_high: uint32, address_low: uint32) -> bool;

pub open spec fn PlatformSupportsStatisticsRegion() -> bool;

} // verus!
