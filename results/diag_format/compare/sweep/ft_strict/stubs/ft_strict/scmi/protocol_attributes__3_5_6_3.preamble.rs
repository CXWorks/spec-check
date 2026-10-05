use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type RsiCommandReturnCode = u32;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub const RSI_INCOMPLETE: RsiCommandReturnCode = 3;

pub struct S {
    pub dummy: int,
}

pub open spec fn Bits(value: UInt32, high: int, low: int) -> UInt32;

pub open spec fn PowerExpressedInMicrowatts() -> bool;

pub open spec fn PowerExpressedInMilliwatts() -> bool;

pub open spec fn PowerExpressedInAbstractLinearScale() -> bool;

pub open spec fn NumPerformanceDomains() -> UInt32;

pub open spec fn StatisticsRegionSupported() -> bool;

pub open spec fn AddrInCallerMemoryMap(addr: int) -> bool;

} // verus!
