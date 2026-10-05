use vstd::prelude::*;
verus! {

pub type unsigned_long = u64;
#[allow(non_camel_case_types)]
pub type long = i64;

pub struct S {
    pub num_harts: u64,
    pub hart_states: Seq<i64>,
}

pub spec const SBI_ERR_INVALID_PARAM: long = -3;

pub open spec fn IsValidHartid(s: S, hartid: unsigned_long) -> bool;

pub open spec fn ResultEqual(a: long, b: long) -> bool;

pub open spec fn IsHsmStateId(value: long) -> bool;

} // verus!
