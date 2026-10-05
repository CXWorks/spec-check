use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub struct S {
    pub attributes: u64,
    pub statistics_len: u64,
    pub statistics_address_high: u32,
    pub statistics_address_low: u32,
}

pub open spec fn Bits(value: u64, hi: u64, lo: u64) -> u64;

pub open spec fn NumPowerDomains() -> u64;

pub open spec fn PlatformSupportsStatisticsRegion() -> bool;

pub open spec fn StatisticsRegionLength() -> u64;

pub open spec fn StatisticsRegionAddress() -> int;

pub open spec fn IsInCallerMemoryMap(addr: int) -> bool;

} // verus!
