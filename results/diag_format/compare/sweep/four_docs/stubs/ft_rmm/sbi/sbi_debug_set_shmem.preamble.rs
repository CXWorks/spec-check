use vstd::prelude::*;

verus! {

pub type unsigned_long = u64;

pub type sbiret = i64;

pub type HartId = u64;

pub struct S {
    pub dummy: u64,
}

pub struct DebugShmemState {
    pub enabled: bool,
    pub base: u64,
    pub size: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub const XLEN: u64 = 64;
pub const trig_max: u64 = 16;

pub open spec fn ResultEqual(result: sbiret, code: i64) -> bool;

pub open spec fn IsAllOnes(s: S, lo: u64, hi: u64) -> bool;

pub open spec fn IsAligned(s: S, addr: u64, align: u64) -> bool;

pub open spec fn ShmemSatisfiesRequirements(s: S, base: u64, size: u64) -> bool;

pub open spec fn RequestFailedForOtherReason(s: S) -> bool;

pub open spec fn DebugShmem(s: S, hart: HartId) -> DebugShmemState;

pub open spec fn CallingHart(s: S) -> HartId;

} // verus!
