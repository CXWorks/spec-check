use vstd::prelude::*;
verus! {

pub type UInt = u64;
pub type SbiErrorCode = i64;

pub struct S {
    pub cppc_reg: u32,
    pub xlen: nat,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_DENIED: SbiErrorCode = -4;

pub open spec fn cppc_reg_id(s: S) -> u32;
pub open spec fn IsReservedCppcReg(s: S, reg_id: u32) -> bool;
pub open spec fn IsImplementedCppcReg(s: S, reg_id: u32) -> bool;
pub open spec fn IsWriteOnlyCppcReg(s: S, reg_id: u32) -> bool;
pub open spec fn CppcReadRequestFailed(s: S, reg_id: u32) -> bool;
pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;
pub open spec fn SupervisorXlen(s: S) -> nat;
pub open spec fn CppcRegValue(s: S, reg_id: u32) -> u64;
pub open spec fn Bits(v: u64, hi: nat, lo: nat) -> UInt;

} // verus!
