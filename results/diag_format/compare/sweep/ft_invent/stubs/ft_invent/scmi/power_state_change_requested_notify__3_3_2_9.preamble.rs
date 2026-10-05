use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub enum RsiCommandReturnCode {
    ErrorInput,
    ErrorState,
    Incomplete,
    ErrorUnknown,
}

pub struct S {
    pub dummy: u64,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());

pub spec const RSI_ERROR_INPUT: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::ErrorInput);

pub spec const RSI_ERROR_STATE: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::ErrorState);

pub spec const RSI_INCOMPLETE: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::Incomplete);

pub spec const RSI_ERROR_UNKNOWN: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::ErrorUnknown);

} // verus!
