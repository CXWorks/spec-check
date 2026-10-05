use vstd::prelude::*;
verus! {

pub type long = i64;
pub type uint32_t = u32;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: long = 0;
pub const SBI_ERR_FAILED: long = -1;
pub const SBI_ERR_INVALID_PARAM: long = -3;

pub open spec fn IsReservedCppcRegId(cppc_reg_id: uint32_t) -> bool;

pub open spec fn CppcProbeFailedForUnspecifiedReason(cppc_reg_id: uint32_t) -> bool;

pub open spec fn IsCppcRegImplemented(cppc_reg_id: uint32_t) -> bool;

pub open spec fn CppcRegWidth(cppc_reg_id: uint32_t) -> long;

pub open spec fn ResultEqual(error: long, code: long) -> bool;

} // verus!
