use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub shmem_size: u64,
}

pub spec const SBI_SUCCESS: i64 = 0;
pub spec const SBI_ERR_INVALID_PARAM: i64 = -3i64;

pub spec const MSG_DATA_MAX_LEN: UInt64 = 4096;

pub uninterp spec fn shared_memory_size(s: S) -> UInt64;

} // verus!
