use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type Int64 = i64;

pub struct SbiRet {
    pub error: Int64,
    pub value: UInt64,
}

pub struct S {
    pub pmu_num_counters: UInt64,
    pub pmu_counter_info: Seq<UInt64>,
}

pub const SBI_SUCCESS: Int64 = 0;

pub const SBI_ERR_INVALID_PARAM: Int64 = -3;

pub open spec fn PmuCounterIsValid(s: S, idx: int) -> bool;

pub open spec fn PmuCounterIsFirmware(s: S, idx: int) -> bool;

pub open spec fn PmuCounterCsr(s: S, idx: int) -> UInt64;

pub open spec fn PmuCounterNumBits(s: S, idx: int) -> UInt64;

} // verus!
