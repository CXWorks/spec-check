use vstd::prelude::*;

verus! {

pub type HartId = u64;

pub type PhysAddress = int;

pub struct S {
    pub flags: u64,
    pub shmem_phys_lo: u64,
    pub shmem_phys_hi: u64,
    pub XLEN: u64,
}

pub spec const SBI_SUCCESS: i64 = 0;
pub spec const SBI_ERR_FAILED: i64 = (-1) as i64;
pub spec const SBI_ERR_INVALID_PARAM: i64 = (-3) as i64;
pub spec const SBI_ERR_INVALID_ADDRESS: i64 = (-5) as i64;

pub open spec fn ResultEqual(error: i64, code: i64) -> bool;

pub open spec fn IsDisableRequest(lo: u64, hi: u64) -> bool;

pub open spec fn PhysAddr(hi: u64, lo: u64) -> PhysAddress;

pub open spec fn ShmemSatisfiesSection3_2Requirements(addr: PhysAddress, size: int) -> bool;

pub open spec fn TrigMax() -> int;

pub open spec fn RequestFailedForUnspecifiedReason() -> bool;

pub open spec fn CallingHart() -> HartId;

pub open spec fn DebugShmemEnabled(s: S, hart: HartId) -> bool;

pub open spec fn DebugShmemBase(s: S, hart: HartId) -> PhysAddress;

pub open spec fn DebugShmemSize(s: S, hart: HartId) -> int;

} // verus!
