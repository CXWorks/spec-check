use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_FAILED: SbiCommandReturnCode = -1;
pub const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiCommandReturnCode = -5;

pub struct S {
    pub nacl_shmem_phys_lo: u64,
    pub nacl_shmem_phys_hi: u64,
}

} // verus!
