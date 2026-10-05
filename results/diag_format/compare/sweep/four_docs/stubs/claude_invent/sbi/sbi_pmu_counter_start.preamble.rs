use vstd::prelude::*;
verus! {

pub struct SbiRet {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_ALREADY_STARTED: i64 = -7;
pub const SBI_ERR_NO_SHMEM: i64 = -9;

pub open spec fn PmuCounterSetHasInvalidCounter(s: S, counter_idx_base: u64, counter_idx_mask: u64) -> bool;

pub open spec fn PmuCounterSetHasStartedCounter(s: S, counter_idx_base: u64, counter_idx_mask: u64) -> bool;

pub open spec fn PmuSnapshotShmemIsSet(s: S) -> bool;

pub open spec fn PmuCounterInSet(counter_idx_base: u64, counter_idx_mask: u64, idx: u64) -> bool;

pub open spec fn PmuCounterIsStarted(s: S, idx: u64) -> bool;

pub open spec fn PmuCounterValue(s: S, idx: u64) -> u64;

pub open spec fn PmuSnapshotCounterValue(s: S, idx: u64) -> u64;

} // verus!
