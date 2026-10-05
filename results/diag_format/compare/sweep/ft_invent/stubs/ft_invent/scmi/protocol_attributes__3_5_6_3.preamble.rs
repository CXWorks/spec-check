use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    RSI_ERROR_INPUT,
    RSI_ERROR_STATE,
    RSI_INCOMPLETE,
    RSI_ERROR_UNKNOWN,
}

pub struct S {
    pub dummy: int,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());

} // verus!
