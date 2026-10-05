use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_FAILED: SbiCommandReturnCode = -1;
pub const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiCommandReturnCode = -5;

pub struct S {
    pub shmem_phys_lo: UInt64,
    pub shmem_phys_hi: UInt64,
    pub flags: UInt64,
}

} // verus!
