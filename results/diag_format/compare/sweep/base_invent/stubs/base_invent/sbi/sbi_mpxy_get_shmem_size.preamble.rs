use vstd::prelude::*;
verus! {

pub type SbiError = i64;

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_DENIED: i64 = -4;

pub spec const MSG_DATA_MAX_LEN: int = 4096;

pub struct sbiret {
    pub error: i64,
    pub uvalue: u64,
}

pub struct S {
    pub shmem_size: u64,
}

} // verus!
