use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub type NaclFeatureId = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub nacl_shmem_set: bool,
    pub hfence_synced: Seq<bool>,
}

pub const XLEN: UInt = 64;

pub const SBI_NACL_FEAT_SYNC_HFENCE: NaclFeatureId = 3;

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;

pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub const SBI_ERR_NO_SHMEM: SbiErrorCode = -9;

pub open spec fn NaclFeatureAvailable(feature: NaclFeatureId) -> bool;

pub open spec fn ResultEqual(result: sbiret, code: SbiErrorCode) -> bool;

pub open spec fn IsAllOnes(value: UInt) -> bool;

pub open spec fn NaclSharedMemoryAvailable() -> bool;

pub open spec fn AllNestedHfenceEntriesSynchronized() -> bool;

pub open spec fn NestedHfenceEntrySynchronized(entry_index: UInt) -> bool;

} // verus!
