use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;

pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub const RSI_INCOMPLETE: RsiCommandReturnCode = 3;

pub const RSI_ERROR_UNKNOWN: RsiCommandReturnCode = 4;

pub const RSI_NOT_FOUND: RsiCommandReturnCode = 5;

pub const RSI_INVALID_PARAMETERS: RsiCommandReturnCode = 6;

pub const RSI_DENIED: RsiCommandReturnCode = 7;

pub const RSI_IN_USE: RsiCommandReturnCode = 8;

pub struct S {
    pub dummy: UInt64,
}

} // verus!
