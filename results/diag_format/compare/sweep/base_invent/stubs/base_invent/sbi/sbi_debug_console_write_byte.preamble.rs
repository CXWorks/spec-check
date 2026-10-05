use vstd::prelude::*;
verus! {

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_DENIED: i64 = -4;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: int,
}

} // verus!
