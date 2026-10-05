use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt = u64;
pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_DENIED: SbiErrorCode = -4;

pub open spec fn IsReservedCppcReg(s: S, reg_id: UInt32) -> bool;
pub open spec fn IsImplementedCppcReg(s: S, reg_id: UInt32) -> bool;
pub open spec fn IsWriteOnlyCppcReg(s: S, reg_id: UInt32) -> bool;
pub open spec fn CppcReadRequestFailed(s: S, reg_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;
pub open spec fn SupervisorXlen(s: S) -> int;
pub open spec fn CppcRegValue(s: S, reg_id: UInt32) -> UInt;
pub open spec fn Bits(v: UInt, hi: int, lo: int) -> UInt;

} // verus!
