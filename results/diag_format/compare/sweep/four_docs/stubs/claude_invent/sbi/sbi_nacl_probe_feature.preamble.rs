use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub struct S {
    pub nacl_features: Seq<bool>,
    pub nacl_shmem: UInt64,
}

pub open spec fn NaclFeatureAvailable(s: S, feature_id: UInt32) -> bool;

} // verus!
