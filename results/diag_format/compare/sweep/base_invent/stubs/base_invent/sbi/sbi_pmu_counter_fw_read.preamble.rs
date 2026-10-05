use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub struct S {
    pub pmu_counter_idx: int,
    pub pmu_counter_fw: Seq<u64>,
}

} // verus!
