use vstd::prelude::*;

verus! {

#[allow(non_camel_case_types)]
pub type unsigned_long = u64;

#[allow(non_camel_case_types)]
pub type uint64_t = u64;

pub struct SbiRet {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub num_counters: u64,
}

pub open spec fn SelectedCounter(s: S) -> u64;

pub open spec fn IsCounterInSet(s: S, counter: u64, counter_idx_base: unsigned_long, counter_idx_mask: unsigned_long) -> bool;

pub open spec fn FirstCounterInSet(s: S, counter_idx_base: unsigned_long, counter_idx_mask: unsigned_long) -> u64;

pub open spec fn CounterWasStarted(s: S, counter: u64) -> bool;

pub open spec fn CounterCanMonitorEvent(s: S, counter: u64, event_idx: unsigned_long) -> bool;

pub open spec fn CounterConfiguredForEvent(s: S, counter: u64, event_idx: unsigned_long, event_data: uint64_t) -> bool;

pub open spec fn CounterValue(s: S, counter: u64) -> u64;

pub open spec fn CounterIsStarted(s: S, counter: u64) -> bool;

pub open spec fn CounterValueUnaffectedByAutoStart(s: S, counter: u64) -> bool;

} // verus!
