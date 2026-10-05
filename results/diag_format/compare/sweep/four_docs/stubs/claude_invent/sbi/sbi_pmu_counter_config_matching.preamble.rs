use vstd::prelude::*;

verus! {

pub struct SbiRet {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub counter_values: Seq<u64>,
    pub counter_started: Seq<bool>,
    pub num_counters: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn CounterSetValid(s: S, counter_idx_base: u64, counter_idx_mask: u64) -> bool;

pub open spec fn ExistsMatchingCounter(s: S, counter_idx_base: u64, counter_idx_mask: u64, event_idx: u64, event_data: u64) -> bool;

pub open spec fn CounterInSet(counter_idx_base: u64, counter_idx_mask: u64, counter: u64) -> bool;

pub open spec fn FirstCounterInSet(counter_idx_base: u64, counter_idx_mask: u64) -> u64;

pub open spec fn CounterStarted(s: S, counter: u64) -> bool;

pub open spec fn CounterCanMonitorEvent(s: S, counter: u64, event_idx: u64, event_data: u64) -> bool;

pub open spec fn CounterConfiguredForEvent(s: S, counter: u64, event_idx: u64, event_data: u64) -> bool;

pub open spec fn CounterValue(s: S, counter: u64) -> u64;

} // verus!
