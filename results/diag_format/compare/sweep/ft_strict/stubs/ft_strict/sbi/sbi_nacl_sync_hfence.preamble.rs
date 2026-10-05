use vstd::prelude::*;

verus! {

pub type UnsignedLong = u64;

pub type SbiErrorCode = i64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub nacl_shmem_available: bool,
    pub nacl_features: u64,
    pub hfence_entries_synced: Seq<bool>,
}

pub const XLEN: u64 = 64;

pub const SBI_NACL_FEAT_SYNC_HFENCE: u64 = 2;

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_NO_SHMEM: i64 = -9;

pub open spec fn NaclFeatureAvailable(s: S, feature: u64) -> bool;

pub open spec fn ResultEqual(result: i64, expected: i64) -> bool;

pub open spec fn IsAllOnes(s: S, value: u64) -> bool;

pub open spec fn NaclSharedMemoryAvailable(s: S) -> bool;

pub open spec fn AllNestedHfenceEntriesSynchronized(s: S) -> bool;

pub open spec fn NestedHfenceEntrySynchronized(s: S, entry_index: u64) -> bool;

} // verus!
