use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct SbiRet {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub regs: Map<u32, u64>,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn CppcRegIdIsReserved(cppc_reg_id: UInt32) -> bool;

pub open spec fn CppcRegIsImplemented(s: S, cppc_reg_id: UInt32) -> bool;

pub open spec fn CppcRegWritten(old_s: S, new_s: S, cppc_reg_id: UInt32, val: UInt64) -> bool;

} // verus!
