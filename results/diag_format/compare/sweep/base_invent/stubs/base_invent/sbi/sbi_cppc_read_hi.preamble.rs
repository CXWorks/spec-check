use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_DENIED: i64 = -4;

pub struct SbiRet {
    pub error: i64,
    pub value: u64,
    pub cppc_reg_id: u32,
}

pub struct S {
    pub xlen: u64,
    pub cppc_reg_id: u32,
}

impl S {
    pub open spec fn cppc_reg_high_bits(self, reg_id: u32) -> u64;
}

pub open spec fn cppc_reg_id_is_reserved(s: S, reg_id: u32) -> bool;

pub open spec fn cppc_reg_id_not_implemented(s: S, reg_id: u32) -> bool;

pub open spec fn cppc_reg_id_is_write_only(s: S, reg_id: u32) -> bool;

} // verus!
