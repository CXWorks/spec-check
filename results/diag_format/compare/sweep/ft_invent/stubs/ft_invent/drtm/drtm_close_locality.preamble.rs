use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_NOT_SUPPORTED: RsiCommandReturnCode = 1;
pub const RSI_INVALID_PARAMETERS: RsiCommandReturnCode = 2;
pub const RSI_ALREADY_CLOSED: RsiCommandReturnCode = 3;
pub const RSI_DENIED: RsiCommandReturnCode = 4;

pub struct S {
    pub dummy: u64,
}

} // verus!
