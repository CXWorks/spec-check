use vstd::prelude::*;
verus! {

pub type uint32_t = u32;
pub type unsigned_long = u64;
#[allow(non_camel_case_types)]
pub type unsigned = u64;

pub type SbiCommandReturnCode = i64;

pub const XLEN: u64 = 64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_FAILED: SbiCommandReturnCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiCommandReturnCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;
pub const SBI_ERR_DENIED: SbiCommandReturnCode = -4;
pub const SBI_ERR_INVALID_ADDRESS: SbiCommandReturnCode = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: SbiCommandReturnCode = -6;
pub const SBI_ERR_ALREADY_STARTED: SbiCommandReturnCode = -7;
pub const SBI_ERR_ALREADY_STOPPED: SbiCommandReturnCode = -8;
pub const SBI_ERR_NO_SHMEM: SbiCommandReturnCode = -9;
pub const SBI_ERR_INVALID_STATE: SbiCommandReturnCode = -10;
pub const SBI_ERR_BAD_RANGE: SbiCommandReturnCode = -11;

pub struct S {
    pub dummy: u64,
}

} // verus!
