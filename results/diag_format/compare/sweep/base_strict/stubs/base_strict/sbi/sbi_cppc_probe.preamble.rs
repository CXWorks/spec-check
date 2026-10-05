use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int64 = i64;

pub struct sbiret {
    pub error: Int64,
    pub value: UInt64,
}

pub struct S {
    pub cppc_regs: Map<UInt32, UInt64>,
}

pub const SBI_SUCCESS: Int64 = 0;
pub const SBI_ERR_FAILED: Int64 = -1;
pub const SBI_ERR_INVALID_PARAM: Int64 = -3;

#[allow(non_upper_case_globals)]
pub const value: UInt64 = 1;

pub open spec fn IsReservedCppcRegId(cppc_reg_id: UInt32) -> bool;

pub open spec fn CppcProbeFailed(cppc_reg_id: UInt32) -> bool;

pub open spec fn IsCppcRegImplemented(cppc_reg_id: UInt32) -> bool;

pub open spec fn CppcRegWidth(cppc_reg_id: UInt32) -> UInt64;

pub open spec fn ResultEqual(result: sbiret, code: Int64) -> bool;

} // verus!
