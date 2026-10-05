use vstd::prelude::*;
verus! {

pub type long = i64;
pub type UInt64 = u64;

pub struct SnapshotShmemT {
    pub counter_value: Map<int, int>,
    pub overflow_bitmap: int,
}

pub struct S {
    pub SnapshotShmem: SnapshotShmemT,
}

pub spec const SBI_SUCCESS: long = 0;
pub spec const SBI_ERR_INVALID_PARAM: long = -3;
pub spec const SBI_ERR_ALREADY_STOPPED: long = -8;
pub spec const SBI_ERR_NO_SHMEM: long = -9;

pub spec const XLEN: int = 64;

pub spec const counter_idx_base: UInt64 = 0;
pub spec const counter_idx_mask: UInt64 = 1;

pub spec const stop_flags: Seq<int> = Seq::empty();

pub spec const SnapshotShmem: SnapshotShmemT = SnapshotShmemT { counter_value: Map::empty(), overflow_bitmap: 0 };

pub open spec fn ResultEqual(result: long, code: long) -> bool;

pub open spec fn AllCountersValid(s: S, base: UInt64, mask: UInt64) -> bool;

pub open spec fn AnyCounterStopped(s: S, base: UInt64, mask: UInt64) -> bool;

pub open spec fn SnapshotShmemAvailable() -> bool;

pub open spec fn CounterSet(base: UInt64, mask: UInt64) -> Set<int>;

pub open spec fn CounterIsStopped(idx: int) -> bool;

pub open spec fn CounterEventMappingIsReset(idx: int) -> bool;

pub open spec fn CounterValue(idx: int) -> int;

pub open spec fn CounterOverflowBitmap() -> int;

} // verus!
