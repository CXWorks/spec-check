use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type RsiCommandReturnCode = u64;

pub spec const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub spec const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub spec const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub spec const RSI_INCOMPLETE: RsiCommandReturnCode = 3;
pub spec const RSI_ERROR_UNKNOWN: RsiCommandReturnCode = 4;

pub struct S {
    pub flags: UInt64,
    pub system_state: UInt64,
    pub timeout: UInt64,
}

} // verus!
