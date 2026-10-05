use vstd::prelude::*;
verus! {

pub type uint32 = u32;

pub enum RsiCommandReturnCode {
    RSI_SUCCESS,
    RSI_ERROR_INPUT,
    RSI_ERROR_STATE,
    RSI_INCOMPLETE,
}

pub struct S {
    pub dummy: int,
}

pub uninterp spec fn Uint32BitSlice(x: u32, hi: int, lo: int) -> int;

pub trait BitSliceIndex {
    spec fn spec_index(self, r: core::ops::Range<int>) -> int;
}

impl BitSliceIndex for u32 {
    open spec fn spec_index(self, r: core::ops::Range<int>) -> int {
        Uint32BitSlice(self, r.start, r.end)
    }
}

pub uninterp spec fn NumPerformanceDomains() -> int;

pub uninterp spec fn IsInCallerMemoryMap(addr_high: uint32, addr_low: uint32) -> bool;

pub uninterp spec fn IsAligned64(addr_high: uint32, addr_low: uint32) -> bool;

pub uninterp spec fn PlatformSupportsStatisticsRegion() -> bool;

} // verus!
