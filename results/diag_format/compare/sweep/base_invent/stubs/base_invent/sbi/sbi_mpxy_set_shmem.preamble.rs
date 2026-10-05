use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub cmd_input_shmem_phys_lo: u64,
    pub cmd_input_shmem_phys_hi: u64,
    pub cmd_input_flags: u64,
    pub shmem_phys_lo: u64,
    pub shmem_phys_hi: u64,
    pub shmem_enabled: bool,
    pub shmem_size: u64,
    pub old_shmem_phys_lo: u64,
    pub old_shmem_phys_hi: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_DENIED: i64 = -4;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: i64 = -6;
pub const SBI_ERR_ALREADY_STARTED: i64 = -7;
pub const SBI_ERR_ALREADY_STOPPED: i64 = -8;
pub const SBI_ERR_NO_SHMEM: i64 = -9;

pub const SBI_MPXY_SHMEM_FLAG_OVERWRITE: u64 = 0;
pub const SBI_MPXY_SHMEM_FLAG_OVERWRITE_RETURN: u64 = 1;

pub open spec fn shmem_size_spec(s: S) -> u64;
pub open spec fn is_valid_shmem_address(lo: u64, hi: u64, size: u64) -> bool;
pub open spec fn shmem_accessible(s: S, lo: u64, hi: u64) -> bool;

} // verus!
