use vstd::prelude::*;
verus! {

pub enum RsiCommandReturnCode {
    RSI_SUCCESS,
    RSI_ERROR_INPUT,
    RSI_ERROR_STATE,
    RSI_INCOMPLETE,
}

pub use RsiCommandReturnCode::*;

pub struct S {
    pub dummy: u64,
}

} // verus!
