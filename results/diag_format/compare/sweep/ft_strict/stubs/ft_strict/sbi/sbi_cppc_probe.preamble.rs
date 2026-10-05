use vstd::prelude::*;
verus! {

pub type uint32_t = u32;

pub type sbiret = i64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn IsReservedCppcRegId(s: S, cppc_reg_id: uint32_t) -> bool;

pub open spec fn CppcProbeFailed(s: S, cppc_reg_id: uint32_t) -> bool;

pub open spec fn IsCppcRegImplemented(s: S, cppc_reg_id: uint32_t) -> bool;

pub open spec fn CppcRegWidth(s: S, cppc_reg_id: uint32_t) -> int;

pub open spec fn ResultEqual(result: sbiret, code: i64) -> bool;

} // verus!
