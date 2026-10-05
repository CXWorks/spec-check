use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    NotFound,
    InvalidParameters,
}

pub struct S {
    pub dummy: u64,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());
pub spec const RSI_ERROR_NOT_FOUND: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::NotFound);
pub spec const RSI_ERROR_INVALID_PARAMETERS: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::InvalidParameters);

} // verus!
