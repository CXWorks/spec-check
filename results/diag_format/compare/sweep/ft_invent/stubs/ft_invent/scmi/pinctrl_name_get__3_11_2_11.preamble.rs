use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    RSI_SUCCESS,
    RSI_ERROR_INPUT,
    RSI_ERROR_STATE,
    RSI_INCOMPLETE,
}

pub use RsiCommandReturnCode::*;

pub struct S {
    pub dummy: int,
}

} // verus!
