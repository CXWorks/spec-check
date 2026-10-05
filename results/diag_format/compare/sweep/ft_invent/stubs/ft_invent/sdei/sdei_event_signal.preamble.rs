use vstd::prelude::*;

verus! {

pub type int32 = i32;

pub type UInt64 = u64;

pub type RsiCommandReturnCode = u64;

pub spec const RSI_SUCCESS: RsiCommandReturnCode = 0;

pub spec const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub spec const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub spec const RSI_ERROR_INVALID_PARAMETERS: bool = false;

pub struct S {
    pub dummy: u64,
}

} // verus!
