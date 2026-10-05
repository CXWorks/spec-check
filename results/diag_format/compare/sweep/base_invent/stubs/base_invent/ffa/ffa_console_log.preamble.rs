use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub struct S {
    pub cmd_input_0: u64,
    pub cmd_input_1: u64,
}

pub spec const FFA_SUCCESS: int = 0;
pub spec const FFA_ERROR_NOT_SUPPORTED: int = -1;
pub spec const FFA_ERROR_INVALID_PARAMETERS: int = -2;
pub spec const FFA_ERROR_RETRY: int = -7;

} // verus!
