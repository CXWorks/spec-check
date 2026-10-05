use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub const RSI_INCOMPLETE: RsiCommandReturnCode = 3;
pub const RSI_ERROR_UNKNOWN: RsiCommandReturnCode = 4;

pub struct S {
    pub placeholder: u64,
}

} // verus!
