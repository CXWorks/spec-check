use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub struct DrtmParameters {
    pub secure_interrupts_disabled: bool,
}

pub struct S {
    pub drtm_parameters: DrtmParameters,
    pub secure_interrupts_enabled: bool,
}

} // verus!
