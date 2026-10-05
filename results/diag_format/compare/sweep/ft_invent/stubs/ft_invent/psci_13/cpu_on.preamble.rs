use vstd::prelude::*;

verus! {

pub type Address = u64;

pub type ContextId = u64;

pub type RsiCommandReturnCode = u64;

pub struct S {
    pub dummy: u64,
}

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INVALID_PARAMETERS: RsiCommandReturnCode = 1;
pub const RSI_ERROR_INVALID_ADDRESS: RsiCommandReturnCode = 2;
pub const RSI_ERROR_ALREADY_ON: RsiCommandReturnCode = 3;
pub const RSI_ERROR_ON_PENDING: RsiCommandReturnCode = 4;
pub const RSI_ERROR_INTERNAL_FAILURE: RsiCommandReturnCode = 5;
pub const RSI_ERROR_DENIED: RsiCommandReturnCode = 6;

} // verus!
