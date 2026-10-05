use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub struct S {
    pub w1: u64,
    pub w2: u64,
    pub w3: u64,
    pub w4: u64,
    pub w5: u64,
    pub w6: u64,
    pub w7: u64,
    pub is_physical_ffa_instance: bool,
}

} // verus!
