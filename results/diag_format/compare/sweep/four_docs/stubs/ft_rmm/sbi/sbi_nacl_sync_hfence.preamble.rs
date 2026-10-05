use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub type NaclFeature = u64;

pub struct S {
    pub nacl_features: Seq<bool>,
    pub nacl_shmem_enabled: bool,
    pub hfence_entries: Seq<u64>,
}

pub const XLEN: u64 = 64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_NO_SHMEM: SbiErrorCode = -9;

pub const SBI_NACL_FEAT_SYNC_HFENCE: NaclFeature = 3;

pub open spec fn NaclFeatureAvailable(s: S, feature: NaclFeature) -> bool;

pub open spec fn IsAllOnes(s: S, value: UInt) -> bool;

pub open spec fn NaclSharedMemoryAvailable(s: S) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

} // verus!
