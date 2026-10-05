use vstd::prelude::*;

verus! {

pub type uint32_t = u32;
pub type long = i64;

pub struct S {
    pub cppc_regs_implemented: Set<u32>,
    pub reserved_cppc_reg_ids: Set<u32>,
}

pub spec const SBI_SUCCESS: long = 0;
pub spec const SBI_ERR_FAILED: long = (-1int) as long;
pub spec const SBI_ERR_INVALID_PARAM: long = (-3int) as long;

#[allow(non_upper_case_globals)]
pub spec const result: long = 1;

pub open spec fn IsReservedCppcRegId(s: S, cppc_reg_id: uint32_t) -> bool;

pub open spec fn CppcProbeFailedForUnspecifiedReason(s: S, cppc_reg_id: uint32_t) -> bool;

pub open spec fn ResultEqual(a: long, b: long) -> bool;

pub open spec fn CppcRegWidth(cppc_reg_id: uint32_t) -> long;

pub open spec fn IsCppcRegImplemented(s: S, cppc_reg_id: uint32_t) -> bool;

} // verus!
