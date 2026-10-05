use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type Int64 = i64;

pub struct sbiret {
    pub error: Int64,
    pub value: UInt64,
}

pub struct S {
    pub xlen: UInt64,
    pub max_physical_address: UInt64,
}

pub const SBI_SUCCESS: Int64 = 0;
pub const SBI_ERR_FAILED: Int64 = -1;
pub const SBI_ERR_INVALID_PARAM: Int64 = -3;
pub const SBI_ERR_INVALID_ADDRESS: Int64 = -5;

pub const flags: UInt64 = 0;
pub const shmem_phys_lo: UInt64 = 1;
pub const shmem_phys_hi: UInt64 = 2;

pub open spec fn valid_shared_memory_address(s: S, shmem_phys_lo: UInt64, shmem_phys_hi: UInt64) -> bool;

} // verus!
