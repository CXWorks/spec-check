use vstd::prelude::*;
verus! {

pub type unsigned_long = u64;
pub type uint64_t = u64;
pub type CounterIdx = u64;

pub struct sbiret {
    pub code: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_PMU_CFG_FLAG_SKIP_MATCH: u64 = 1;
pub const SBI_PMU_CFG_FLAG_CLEAR_VALUE: u64 = 2;
pub const SBI_PMU_CFG_FLAG_AUTO_START: u64 = 4;

pub open spec fn selected_counter(s: S) -> CounterIdx;

pub open spec fn CounterInSet(s: S, counter: CounterIdx, counter_idx_base: unsigned_long, counter_idx_mask: unsigned_long) -> bool;

pub open spec fn FlagSet(s: S, flags: unsigned_long, flag: u64) -> bool;

pub open spec fn CounterStarted(s: S, counter: CounterIdx) -> bool;

pub open spec fn CounterCanMonitor(s: S, counter: CounterIdx, event_idx: unsigned_long) -> bool;

pub open spec fn FirstCounterInSet(s: S, counter_idx_base: unsigned_long, counter_idx_mask: unsigned_long) -> CounterIdx;

pub open spec fn CounterEvent(s: S, counter: CounterIdx) -> unsigned_long;

pub open spec fn CounterEventData(s: S, counter: CounterIdx) -> uint64_t;

pub open spec fn CounterValue(s: S, counter: CounterIdx) -> u64;

} // verus!
