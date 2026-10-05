use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    RSI_SUCCESS,
    RSI_ERROR_INPUT,
    RSI_ERROR_STATE,
    RSI_INCOMPLETE,
}

pub const RSI_SUCCESS: RsiCommandReturnCode = RsiCommandReturnCode::RSI_SUCCESS;

pub struct S {
    pub dummy: int,
}

} // verus!
