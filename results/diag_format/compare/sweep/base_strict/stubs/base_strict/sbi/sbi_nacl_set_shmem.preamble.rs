use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;

pub const XLEN: u64 = 64;

pub struct S {
    pub nacl_shmem_enabled: bool,
    pub nacl_shmem_base: int,
    pub nacl_features_enabled: bool,
}

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsAllOnes(x: UInt) -> bool;

pub open spec fn ShmemBase(lo: UInt, hi: UInt) -> int;

pub open spec fn ShmemSatisfiesRequirements(base: int, size: int) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason() -> bool;

pub open spec fn NaclShmemEnabled(s: S) -> bool;

pub open spec fn NaclShmemBase(s: S) -> int;

pub open spec fn NaclFeaturesEnabled(s: S) -> bool;

} // verus!
