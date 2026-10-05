use vstd::prelude::*;
verus! {

pub type uint32_t = u32;
pub type unsigned_long = u64;
#[allow(non_camel_case_types)]
pub type unsigned = u64;

pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiCommandReturnCode = -5;

pub const XLEN: u64 = 64;

pub struct S {
    pub dummy: u64,
}

} // verus!
