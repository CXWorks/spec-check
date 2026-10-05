use vstd::prelude::*;
verus! {

pub const SBI_SUCCESS: i64 = 0;

pub struct S {
    pub num_counters: u64,
}

pub open spec fn PmuNumCounters(s: S) -> u64;

} // verus!
