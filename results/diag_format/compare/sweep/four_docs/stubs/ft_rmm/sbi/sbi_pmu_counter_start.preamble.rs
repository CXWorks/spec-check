use vstd::prelude::*;
verus! {

pub type unsigned_long = u64;

pub type uint64_t = u64;

pub type SbiCommandReturnCode = u64;

pub const RSI_SUCCESS: SbiCommandReturnCode = 0;

pub const SBI_PMU_START_SET_INIT_VALUE: unsigned_long = 1;

pub const SBI_PMU_START_FLAG_INIT_SNAPSHOT: unsigned_long = 2;

pub mod RsiCommandReturnCode {
    pub const RSI_ERROR_INPUT: bool = false;
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn CounterSet(s: S, counter_idx_base: unsigned_long, counter_idx_mask: unsigned_long) -> Set<u64>;

pub open spec fn ValidCounters(s: S, counters: Set<u64>) -> Set<u64>;

pub open spec fn CounterIsStarted(s: S, idx: u64) -> bool;

pub open spec fn CounterValue(s: S, idx: u64) -> uint64_t;

pub open spec fn SnapshotShmemCounterValue(s: S, idx: u64) -> uint64_t;

} // verus!
