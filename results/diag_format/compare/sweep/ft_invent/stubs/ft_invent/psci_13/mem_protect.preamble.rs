use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub struct S {
    pub dummy: u64,
}

} // verus!
