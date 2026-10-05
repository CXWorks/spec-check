use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub type PdevState = u64;

pub const PDEV_NORMAL: PdevState = 0;
pub const PDEV_SECURE: PdevState = 1;

pub struct S {
    pub pdev: PdevState,
}

} // verus!
