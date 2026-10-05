use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub enum RsiCommandReturnCode {
    Success,
    NotFound,
    OutOfRange,
    Denied,
}

pub struct S {
    pub dummy: int,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());
pub spec const SUCCESS: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::Success);
pub spec const NOT_FOUND: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::NotFound);
pub spec const OUT_OF_RANGE: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::OutOfRange);

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;

pub open spec fn IsValidRateIndex(s: S, clock_id: UInt32, rate_index: UInt32) -> bool;

pub open spec fn ResultEqual(r1: Result<(), RsiCommandReturnCode>, r2: Result<(), RsiCommandReturnCode>) -> bool;

} // verus!
