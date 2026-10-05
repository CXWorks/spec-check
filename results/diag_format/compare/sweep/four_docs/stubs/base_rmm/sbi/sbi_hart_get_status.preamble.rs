use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn IsValidHartid(hartid: u64) -> bool;

pub open spec fn IsHsmStateId(value: i64) -> bool;

} // verus!
