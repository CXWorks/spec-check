use vstd::prelude::*;

verus! {

pub type Address = u64;

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub type HartId = u64;

#[allow(non_camel_case_types)]
pub type shmem_phys_lo = Address;

pub struct S {
    pub dummy: int,
}

pub spec const SBI_SUCCESS: SbiErrorCode = 0;

pub spec const SBI_ERR_FAILED: SbiErrorCode = -1;

pub spec const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub spec const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;

pub open spec fn ResultEqual(result: SbiErrorCode, code: SbiErrorCode) -> bool;

pub open spec fn IsAllOnes(s: S, lo: Address, hi: Address) -> bool;

pub open spec fn AddrIsAligned(s: S, addr: Address, align: int) -> bool;

pub open spec fn ShmemSatisfiesRequirements(s: S, addr: Address, size: int) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(s: S) -> bool;

pub open spec fn NaclShmemBase(s: S, hart: HartId) -> Address;

pub open spec fn NaclEnabled(s: S, hart: HartId) -> bool;

pub open spec fn CallingHart() -> HartId;

} // verus!
