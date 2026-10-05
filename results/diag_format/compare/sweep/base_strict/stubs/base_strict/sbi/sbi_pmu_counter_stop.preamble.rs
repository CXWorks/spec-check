use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type PmuCounterStopFlags = u64;

pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: u64,
}

pub const XLEN: u64 = 64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_ALREADY_STOPPED: SbiErrorCode = -8;
pub const SBI_ERR_NO_SHMEM: SbiErrorCode = -9;

pub open spec fn CounterInSet(counter_idx_base: UInt, counter_idx_mask: UInt, i: UInt) -> bool;

pub open spec fn IsValidCounter(i: UInt) -> bool;

pub open spec fn ResultEqual(result: SbiErrorCode, code: SbiErrorCode) -> bool;

pub open spec fn Bits(value: PmuCounterStopFlags, hi: int, lo: int) -> int;

pub open spec fn CounterIsStopped(i: UInt) -> bool;

pub open spec fn SnapshotShmemAvailable() -> bool;

pub open spec fn CounterEventMappingIsReset(i: UInt) -> bool;

pub open spec fn SnapshotShmemCounterValue(i: UInt) -> int;

pub open spec fn CounterValue(i: UInt) -> int;

pub open spec fn SnapshotShmemCounterValueUnchanged(i: UInt) -> bool;

pub open spec fn SnapshotShmemOverflowBitmapUpdated() -> bool;

} // verus!
