use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_ALREADY_STOPPED: SbiErrorCode = -8;
pub const SBI_ERR_NO_SHMEM: SbiErrorCode = -9;

pub struct S {
    pub dummy: int,
}

pub open spec fn CounterSetHasInvalidCounter(s: S, counter_idx_base: UInt64, counter_idx_mask: UInt64) -> bool;

pub open spec fn CounterSetHasStoppedCounter(s: S, counter_idx_base: UInt64, counter_idx_mask: UInt64) -> bool;

pub open spec fn SnapshotShmemAvailable(s: S) -> bool;

pub open spec fn CounterIsStopped(s: S, idx: int) -> bool;

pub open spec fn CounterEventMappingIsReset(s: S, idx: int) -> bool;

pub open spec fn SnapshotCounterValue(s: S, idx: int) -> u64;

pub open spec fn CounterValue(s: S, idx: int) -> u64;

pub open spec fn SnapshotOverflowBitmapUpdated(old_s: S, new_s: S) -> bool;

} // verus!
