use vstd::prelude::*;
verus! {

pub enum RsiCommandReturnCode {
    RSI_SUCCESS,
    RSI_ERROR_STATE,
    DENIED,
}

pub struct S {
    pub drtm_tcb_hashes_locked: bool,
}

} // verus!
