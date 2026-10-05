use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct SbiRet {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_NO_SHMEM: i64 = -9;

pub const SBI_NACL_FEAT_SYNC_HFENCE: u64 = 2;

pub open spec fn NaclFeatureAvailable(s: S, feature: u64) -> bool;

pub open spec fn NaclShmemAvailable(s: S) -> bool;

pub open spec fn NaclAllHfenceEntriesSynchronized(old_s: S, new_s: S) -> bool;

pub open spec fn NaclHfenceEntrySynchronized(old_s: S, new_s: S, entry_index: UInt64) -> bool;

} // verus!
