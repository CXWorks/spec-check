use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int64 = i64;

pub const SBI_SUCCESS: i64 = 0;

pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub struct SbiRet {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub powered_on: bool,
    pub reset_pending: bool,
}

} // verus!
