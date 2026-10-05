use vstd::prelude::*;
verus! {

pub struct S {
    pub cmd_input_flags: u64,
    pub cmd_input_shmem_phys_lo: u64,
    pub cmd_input_shmem_phys_hi: u64,
}

pub const SBI_SUCCESS: i32 = 0;
pub const SBI_ERR_FAILED: i32 = -1;
pub const SBI_ERR_INVALID_PARAM: i32 = -3;
pub const SBI_ERR_INVALID_ADDRESS: i32 = -5;

pub const XLEN: u64 = 64;

#[allow(non_upper_case_globals)]
pub const flags: u64 = 0;
#[allow(non_upper_case_globals)]
pub const shmem_phys_lo: u64 = 4096;
#[allow(non_upper_case_globals)]
pub const shmem_phys_hi: u64 = 0;

pub uninterp spec fn valid_shared_memory_address(shmem_phys_lo: u64, shmem_phys_hi: u64) -> bool;

} // verus!
