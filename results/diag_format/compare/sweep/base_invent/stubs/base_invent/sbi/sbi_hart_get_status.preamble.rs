use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

impl sbiret {
    pub open spec fn is_Ok(&self) -> bool;
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn ResultEqual(result: sbiret, code: i64) -> bool;

} // verus!
