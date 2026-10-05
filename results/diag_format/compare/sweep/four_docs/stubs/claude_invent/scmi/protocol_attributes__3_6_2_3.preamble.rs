use vstd::prelude::*;

verus! {

pub struct S {
    pub clock_max_pending_async_rate_changes: int,
    pub clock_number_of_clocks: int,
}

pub open spec fn ClockMaxPendingAsyncRateChanges(s: S) -> int;

pub open spec fn ClockNumberOfClocks(s: S) -> int;

} // verus!
