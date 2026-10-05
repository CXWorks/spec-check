use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: int,
}

pub const XLEN: u64 = 64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_NO_SHMEM: SbiErrorCode = -9;

pub const SBI_NACL_FEAT_SYNC_HFENCE: u64 = 2;

pub open spec fn NaclFeatureAvailable(feature: u64) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsAllOnes(x: UInt) -> bool;

pub open spec fn NaclSharedMemoryAvailable() -> bool;

pub open spec fn AllNestedHfenceEntriesSynchronized(old_s: S, new_s: S) -> bool;

pub open spec fn NestedHfenceEntrySynchronized(old_s: S, new_s: S, entry_index: UInt) -> bool;

} // verus!
