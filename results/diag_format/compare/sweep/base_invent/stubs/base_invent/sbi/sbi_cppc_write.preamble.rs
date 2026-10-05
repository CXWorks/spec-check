use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int64 = i64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
    pub cppc_reg_id: u32,
}

pub struct S {
    pub cppc_registers: Map<u32, u64>,
    pub val: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn cppc_reg_id_is_reserved(s: S, reg_id: u32) -> bool;
pub open spec fn cppc_reg_id_not_implemented(s: S, reg_id: u32) -> bool;
pub open spec fn cppc_reg_id_not_reserved(s: S, reg_id: u32) -> bool;
pub open spec fn cppc_reg_id_implemented(s: S, reg_id: u32) -> bool;

} // verus!
