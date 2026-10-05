use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;

pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub spec const null_ptr: *const u8 = vstd::raw_ptr::ptr_null();

pub struct S {
    pub dummy: u64,
}

} // verus!
