use vstd::prelude::*;
verus! {

pub type UInt = u64;
pub type PmuCounterStopFlags = u64;
pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: u64,
}

pub const XLEN: UInt = 64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_DENIED: SbiErrorCode = -4;
pub const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: SbiErrorCode = -6;
pub const SBI_ERR_ALREADY_STARTED: SbiErrorCode = -7;
pub const SBI_ERR_ALREADY_STOPPED: SbiErrorCode = -8;
pub const SBI_ERR_NO_SHMEM: SbiErrorCode = -9;

pub open spec fn CounterInSet(s: S, counter_idx_base: UInt, counter_idx_mask: UInt, i: UInt) -> bool;
pub open spec fn IsValidCounter(s: S, i: UInt) -> bool;
pub open spec fn ResultEqual(result: SbiErrorCode, code: SbiErrorCode) -> bool;
pub open spec fn Bits(x: PmuCounterStopFlags, hi: int, lo: int) -> int;
pub open spec fn CounterIsStopped(s: S, i: UInt) -> bool;
pub open spec fn SnapshotShmemAvailable(s: S) -> bool;
pub open spec fn CounterEventMappingIsReset(s: S, i: UInt) -> bool;
pub open spec fn SnapshotShmemCounterValue(s: S, i: UInt) -> u64;
pub open spec fn CounterValue(s: S, i: UInt) -> u64;
pub open spec fn SnapshotShmemCounterValueUnchanged(s: S, i: UInt) -> bool;
pub open spec fn SnapshotShmemOverflowBitmapUpdated(s: S) -> bool;

} // verus!
