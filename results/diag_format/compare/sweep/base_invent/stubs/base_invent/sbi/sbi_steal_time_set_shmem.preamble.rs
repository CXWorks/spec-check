use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

impl sbiret {
    pub open spec fn is_Ok(&self) -> bool;
}

pub struct S {
    pub cmd_input_shmem_phys_lo: u64,
    pub cmd_input_shmem_phys_hi: u64,
    pub cmd_input_flags: u64,
    pub steal_time_shmem_base: u64,
    pub steal_time_shmem_enabled: bool,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERROR_FAILED: i64 = -1;
pub const SBI_ERROR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERROR_INVALID_PARAM: i64 = -3;
pub const SBI_ERROR_DENIED: i64 = -4;
pub const SBI_ERROR_INVALID_ADDRESS: i64 = -5;

pub open spec fn ResultEqual(result: sbiret, code: i64) -> bool;

} // verus!
