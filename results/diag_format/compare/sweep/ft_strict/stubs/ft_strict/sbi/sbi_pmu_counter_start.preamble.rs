use vstd::prelude::*;
verus! {

pub type unsigned_long = u64;
#[allow(non_camel_case_types)]
pub type unsigned = u32;
pub type uint64_t = u64;
pub type CounterIndex = u64;
pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_FAILED: SbiCommandReturnCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiCommandReturnCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;
pub const SBI_ERR_ALREADY_STARTED: SbiCommandReturnCode = -7;
pub const SBI_ERR_NO_SHMEM: SbiCommandReturnCode = -9;

pub struct S {
    pub dummy: u64,
}

pub open spec fn InCounterSet(counter_idx_base: unsigned_long, counter_idx_mask: unsigned_long, i: CounterIndex) -> bool;
pub open spec fn CounterStarted(s: S, i: CounterIndex) -> bool;
pub open spec fn Bits(x: unsigned_long, hi: int, lo: int) -> unsigned_long;
pub open spec fn CounterValue(s: S, i: CounterIndex) -> uint64_t;
pub open spec fn IsValidCounter(s: S, i: CounterIndex) -> bool;
pub open spec fn SnapshotShmemCounterValue(s: S, i: CounterIndex) -> uint64_t;
pub open spec fn PreCounterValue(s: S, i: CounterIndex) -> uint64_t;

} // verus!
