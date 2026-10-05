use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub error_code: u32,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn cppc_reg_id_is_reserved(s: S, reg_id: u32) -> bool;

} // verus!
