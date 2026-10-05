use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    NotFound,
    InvalidParameters,
    GenericError,
    Denied,
}

pub struct S {
    pub dummy: u64,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());
pub spec const RSI_ERROR_NOT_FOUND: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::NotFound);
pub spec const RSI_ERROR_INVALID_PARAMETERS: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::InvalidParameters);
pub spec const RSI_ERROR_GENERIC_ERROR: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::GenericError);
pub spec const RSI_ERROR_DENIED: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::Denied);

} // verus!
