use vstd::prelude::*;
verus! {

pub type SbiErrorCode = i64;
pub type UInt64 = u64;
pub type HartId = u64;

pub struct S {
    pub flags: u64,
    pub shmem_phys_lo: u64,
    pub shmem_phys_hi: u64,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;

pub const XLEN: u64 = 64;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsAllOnes(lo: u64, hi: u64) -> bool;

pub open spec fn AddrIsAligned(addr: u64, align: int) -> bool;

pub open spec fn ShmemSatisfiesRequirements(addr: u64, size: int) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason() -> bool;

pub open spec fn CallingHart() -> HartId;

pub open spec fn NaclShmemBase(hart: HartId) -> u64;

pub open spec fn NaclEnabled(hart: HartId) -> bool;

} // verus!
