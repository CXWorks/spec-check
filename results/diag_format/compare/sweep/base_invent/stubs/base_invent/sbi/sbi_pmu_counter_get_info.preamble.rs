use vstd::prelude::*;

verus! {

pub const XLEN: u64 = 64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub pmu_counter_count: u64,
}

} // verus!
