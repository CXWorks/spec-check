use vstd::prelude::*;

verus! {

pub type UnsignedLong = u64;

pub enum SbiStatusCode {
    Success,
    ErrFailed,
    ErrNotSupported,
    ErrInvalidParam,
    ErrDenied,
    ErrInvalidAddress,
    ErrAlreadyAvailable,
    ErrAlreadyStarted,
    ErrAlreadyStopped,
    ErrNoShmem,
}

pub struct SnapshotShmemState {
    pub counter_value: Map<UnsignedLong, u64>,
    pub overflow_bitmap: u64,
}

pub struct S {
    pub snapshot_shmem: SnapshotShmemState,
    pub counters_stopped: Map<UnsignedLong, bool>,
    pub counters_value: Map<UnsignedLong, u64>,
}

pub spec const SBI_SUCCESS: Result<(), SbiStatusCode> = Ok(());
pub spec const SBI_ERR_INVALID_PARAM: Result<(), SbiStatusCode> = Err(SbiStatusCode::ErrInvalidParam);
pub spec const SBI_ERR_ALREADY_STOPPED: Result<(), SbiStatusCode> = Err(SbiStatusCode::ErrAlreadyStopped);
pub spec const SBI_ERR_NO_SHMEM: Result<(), SbiStatusCode> = Err(SbiStatusCode::ErrNoShmem);

pub open spec fn AllCountersValid(s: S, counter_idx_base: UnsignedLong, counter_idx_mask: UnsignedLong) -> bool;

pub open spec fn AnyCounterStopped(s: S, counter_idx_base: UnsignedLong, counter_idx_mask: UnsignedLong) -> bool;

pub open spec fn SnapshotShmemAvailable(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<(), SbiStatusCode>, expected: Result<(), SbiStatusCode>) -> bool;

pub open spec fn CounterSet(s: S, counter_idx_base: UnsignedLong, counter_idx_mask: UnsignedLong) -> Set<UnsignedLong>;

pub open spec fn CounterIsStopped(s: S, idx: UnsignedLong) -> bool;

pub open spec fn CounterEventMappingIsReset(s: S, idx: UnsignedLong) -> bool;

pub open spec fn SnapshotShmem(s: S) -> SnapshotShmemState;

pub open spec fn CounterValue(s: S, idx: UnsignedLong) -> u64;

pub open spec fn CounterOverflowBitmap(s: S) -> u64;

} // verus!
