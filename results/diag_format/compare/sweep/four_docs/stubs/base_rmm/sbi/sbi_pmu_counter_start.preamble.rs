use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub enum sbiret {
    SBI_SUCCESS,
    SBI_ERR_FAILED,
    SBI_ERR_NOT_SUPPORTED,
    SBI_ERR_INVALID_PARAM,
    SBI_ERR_DENIED,
    SBI_ERR_INVALID_ADDRESS,
    SBI_ERR_ALREADY_AVAILABLE,
    SBI_ERR_ALREADY_STARTED,
    SBI_ERR_ALREADY_STOPPED,
    SBI_ERR_NO_SHMEM,
}

pub struct S {
    pub a0: UInt64,
    pub a1: UInt64,
    pub a2: UInt64,
    pub a3: UInt64,
    pub a4: UInt64,
    pub a5: UInt64,
    pub a6: UInt64,
    pub a7: UInt64,
    pub shmem_set: bool,
}

pub spec const SBI_PMU_START_SET_INIT_VALUE: int = 1;
pub spec const SBI_PMU_START_FLAG_INIT_SNAPSHOT: int = 2;

pub open spec fn CounterSet(counter_idx_base: int, counter_idx_mask: int) -> Set<int>;

pub open spec fn ValidCounters(counters: Set<int>) -> Set<int>;

pub open spec fn CounterIsStarted(idx: int, s: S) -> bool;

pub open spec fn CounterValue(idx: int, s: S) -> int;

pub open spec fn SnapshotShmemCounterValue(idx: int) -> int;

} // verus!
