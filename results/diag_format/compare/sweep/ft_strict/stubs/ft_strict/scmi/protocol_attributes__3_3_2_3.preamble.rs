use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub struct S {
    pub dummy: int,
}

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> int;

pub open spec fn NumPowerDomains() -> int;

pub open spec fn PlatformSupportsStatisticsRegion() -> bool;

pub open spec fn StatisticsRegionLength() -> UInt32;

pub open spec fn StatisticsRegionAddress() -> int;

pub open spec fn IsInCallerMemoryMap(addr: int) -> bool;

} // verus!
