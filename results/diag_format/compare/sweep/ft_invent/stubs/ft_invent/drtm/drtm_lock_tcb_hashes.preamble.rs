use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_DENIED: RsiCommandReturnCode = 1;

pub struct S {
    pub drtm_tcb_hashes_locked: bool,
}

pub open spec fn DRTM_TCB_HASHES_LOCKED(s: S) -> bool;

} // verus!
