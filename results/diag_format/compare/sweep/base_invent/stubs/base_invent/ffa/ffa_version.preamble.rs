use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type uint64 = u64;

pub struct S {
    pub cmd_input_w0: u64,
    pub cmd_input_w1: u64,
    pub cmd_input_w2: u64,
    pub cmd_input_w3: u64,
    pub cmd_output_w0: u64,
    pub cmd_output_w1: u64,
    pub cmd_output_w2: u64,
    pub cmd_output_w3: u64,
}

pub spec const SMCCC_SUCCESS: int32 = 0;
pub spec const SMCCC_NOT_SUPPORTED: int32 = -1;
pub spec const SMCCC_NOT_REQUIRED: int32 = -2;
pub spec const SMCCC_INVALID_PARAMETER: int32 = -3;

pub open spec fn ResultEqual(result: int32, expected: int32) -> bool;

} // verus!
