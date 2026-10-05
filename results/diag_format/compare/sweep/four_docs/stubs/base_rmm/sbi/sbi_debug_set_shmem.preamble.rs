use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type HartId = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub flags: u64,
    pub shmem_phys_lo: u64,
    pub shmem_phys_hi: u64,
}

pub struct DebugShmemState {
    pub enabled: bool,
    pub base: u64,
    pub size: int,
}

pub const TRUE: bool = true;

pub const FALSE: bool = false;

pub const XLEN: u64 = 64;

pub const trig_max: u64 = 16;

pub const SBI_SUCCESS: i64 = 0;

pub const SBI_ERR_FAILED: i64 = -1;

pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub open spec fn ResultEqual(error: sbiret, code: i64) -> bool;

pub open spec fn IsAllOnes(lo: u64, hi: u64) -> bool;

pub open spec fn IsAligned(addr: u64, align: u64) -> bool;

pub open spec fn ShmemSatisfiesRequirements(hi: u64, lo: u64, size: int) -> bool;

pub open spec fn RequestFailedForOtherReason() -> bool;

pub open spec fn CallingHart() -> HartId;

pub open spec fn DebugShmem(hart: HartId) -> DebugShmemState;

} // verus!
