use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub uvalue: u64,
}

pub struct S {
    pub shmem_start_index: u64,
    pub shmem_channel_count: u64,
    pub shmem_returned: u64,
    pub shmem_remaining: u64,
    pub shmem_enabled: bool,
    pub shmem_accessible: bool,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_DENIED: i64 = -4;
pub const SBI_ERR_NO_SHMEM: i64 = -9;

} // verus!
