use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

#[derive(PartialEq, Eq)]
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
