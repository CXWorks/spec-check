use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INVALID_PARAMETERS: RsiCommandReturnCode = 1;
pub const RSI_ERROR_NOT_SUPPORTED: RsiCommandReturnCode = 2;
pub const RSI_ERROR_DENIED: RsiCommandReturnCode = 3;
pub const RSI_ERROR_ALREADY_CLOSED: RsiCommandReturnCode = 4;

pub struct S {
    pub drtm_supported: bool,
    pub drtm_denied: bool,
}

impl S {
    pub uninterp spec fn drtm_locality_relinquished(self, locality: int) -> bool;
}

} // verus!
