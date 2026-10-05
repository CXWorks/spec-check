use vstd::prelude::*;

verus! {

#[allow(non_camel_case_types)]
pub type unsigned_long = u64;

pub type SbiErrorCode = i64;

pub type HartId = u64;

pub type PhysAddr = u64;

pub struct S {
    pub dummy: int,
}

pub spec const SBI_SUCCESS: SbiErrorCode = 0;
pub spec const SBI_ERR_FAILED: SbiErrorCode = -1;
pub spec const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub spec const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsAllOnes(s: S, v: unsigned_long) -> bool;

pub open spec fn ShmemBase(s: S, lo: unsigned_long, hi: unsigned_long) -> PhysAddr;

pub open spec fn ShmemSatisfiesRequirements(s: S, base: PhysAddr, size: int) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(s: S) -> bool;

pub open spec fn CurrentHart(s: S) -> HartId;

pub open spec fn NaclShmemEnabled(s: S, hart: HartId) -> bool;

pub open spec fn NaclShmemBase(s: S, hart: HartId) -> PhysAddr;

pub open spec fn NaclFeaturesEnabled(s: S, hart: HartId) -> bool;

} // verus!
