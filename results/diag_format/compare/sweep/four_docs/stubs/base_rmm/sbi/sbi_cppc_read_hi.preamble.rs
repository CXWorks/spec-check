use vstd::prelude::*;
verus! {

pub type UInt = u64;
pub type UInt32 = u32;
pub type SbiErrorCode = i64;

pub struct S {
    pub cppc_regs: Map<UInt32, UInt>,
    pub implemented: Set<UInt32>,
}

pub const cppc_reg_id: UInt32 = 0;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_DENIED: SbiErrorCode = -4;

pub open spec fn IsReservedCppcReg(s: S, reg_id: UInt32) -> bool;
pub open spec fn IsImplementedCppcReg(s: S, reg_id: UInt32) -> bool;
pub open spec fn IsWriteOnlyCppcReg(s: S, reg_id: UInt32) -> bool;
pub open spec fn CppcRegReadFailed(s: S, reg_id: UInt32) -> bool;
pub open spec fn CppcRegValue(s: S, reg_id: UInt32) -> UInt;
pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;
pub open spec fn SupervisorXlen() -> int;

} // verus!
