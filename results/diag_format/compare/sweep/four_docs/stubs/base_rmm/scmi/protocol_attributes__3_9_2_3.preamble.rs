use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub const RSI_INCOMPLETE: RsiCommandReturnCode = 3;

pub struct S {
    pub dummy: u64,
}

pub open spec fn NumVoltageDomains() -> u64;

} // verus!
