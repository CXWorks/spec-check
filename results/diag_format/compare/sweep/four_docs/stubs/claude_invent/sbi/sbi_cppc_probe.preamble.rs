use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub struct S {
    pub cppc_regs: Map<UInt32, UInt64>,
}

pub open spec fn CppcRegIdIsReserved(cppc_reg_id: UInt32) -> bool;

pub open spec fn CppcRegIsImplemented(s: S, cppc_reg_id: UInt32) -> bool;

pub open spec fn CppcRegWidth(s: S, cppc_reg_id: UInt32) -> UInt64;

} // verus!
