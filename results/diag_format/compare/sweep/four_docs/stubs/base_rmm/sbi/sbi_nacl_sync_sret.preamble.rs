use vstd::prelude::*;

verus! {

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_NACL_SYNC_SRET_ERROR_INPUT: SbiErrorCode = -1;
pub const SBI_NACL_SYNC_SRET_ERROR_STATE: SbiErrorCode = -2;
pub const SBI_NACL_SYNC_SRET_ERROR_UNKNOWN: SbiErrorCode = -3;

pub struct S {
    pub dummy: u64,
}

impl S {
    pub open spec fn NaclSharedMemCsrsSynchronized(self) -> bool;
    pub open spec fn NaclSharedMemHfencesSynchronized(self) -> bool;
    pub open spec fn SretEmulated(self) -> bool;
}

pub open spec fn FunctionDoesNotReturn() -> bool;

} // verus!
