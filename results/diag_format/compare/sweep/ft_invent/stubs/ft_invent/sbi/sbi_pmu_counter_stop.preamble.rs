use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;
pub const SBI_ERR_ALREADY_STOPPED: SbiCommandReturnCode = -8;
pub const SBI_ERR_NO_SHMEM: SbiCommandReturnCode = -9;

pub const SBI_PMU_STOP_FLAG_RESET: UInt64 = 1;
pub const SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT: UInt64 = 2;

pub struct S {
    pub dummy: u64,
}

} // verus!
