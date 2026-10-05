use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type Int64 = i64;

pub struct SbiRet {
    pub error: Int64,
    pub value: Int64,
}

pub struct S {
    pub num_harts: UInt64,
    pub hart_states: Seq<Int64>,
}

pub const SBI_SUCCESS: Int64 = 0;

pub const SBI_ERR_INVALID_PARAM: Int64 = -3;

pub open spec fn IsValidHartId(s: S, hartid: UInt64) -> bool;

pub open spec fn IsValidHsmStateId(v: Int64) -> bool;

} // verus!
