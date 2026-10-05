use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub const RSI_INCOMPLETE: RsiCommandReturnCode = 3;

pub struct S {
    pub attributes: u32,
    pub statistics_len: u32,
    pub statistics_address_low: u32,
    pub statistics_address_high: u32,
}

pub open spec fn Bits(value: u32, high: int, low: int) -> u32;

pub open spec fn PowerExpressedInMicrowatts() -> bool;

pub open spec fn PowerExpressedInMilliwatts() -> bool;

pub open spec fn PowerExpressedInAbstractLinearScale() -> bool;

pub open spec fn NumPerformanceDomains() -> u32;

pub open spec fn StatisticsRegionSupported() -> bool;

pub open spec fn AddrInCallerMemoryMap(addr: int) -> bool;

} // verus!
