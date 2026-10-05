use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type Int64 = i64;

pub struct SbiRet {
    pub error: Int64,
    pub value: UInt64,
}

pub struct S {
    pub dummy: UInt64,
}

pub const SBI_SUCCESS: Int64 = 0;

pub const SBI_ERR_INVALID_PARAM: Int64 = -3;

} // verus!
