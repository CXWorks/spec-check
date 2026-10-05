use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct SbiRet {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_DENIED: i64 = -4;
pub const SBI_ERR_FAILED: i64 = -1;

} // verus!
