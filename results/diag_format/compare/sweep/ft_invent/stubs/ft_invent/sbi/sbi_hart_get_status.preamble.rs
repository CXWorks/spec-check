use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Int64 = i64;

pub struct SbiRet {
    pub error: Int64,
    pub value: Int64,
}

pub struct S {
    pub dummy: UInt64,
}

pub spec const SBI_SUCCESS: Int64 = 0;
pub spec const SBI_ERR_FAILED: Int64 = (-1) as Int64;
pub spec const SBI_ERR_NOT_SUPPORTED: Int64 = (-2) as Int64;
pub spec const SBI_ERR_INVALID_PARAM: Int64 = (-3) as Int64;
pub spec const SBI_ERR_DENIED: Int64 = (-4) as Int64;
pub spec const SBI_ERR_INVALID_ADDRESS: Int64 = (-5) as Int64;
pub spec const SBI_ERR_ALREADY_AVAILABLE: Int64 = (-6) as Int64;

} // verus!
