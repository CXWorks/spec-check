use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub spec const SBI_SUCCESS: i64 = 0;
pub spec const SBI_ERR_NOT_SUPPORTED: i64 = (-2int) as i64;
pub spec const SBI_ERR_INVALID_PARAM: i64 = (-3int) as i64;

pub spec const cppc_reg_id: u32 = 7;
pub spec const val: u64 = 42;

pub uninterp spec fn IsReservedCppcReg(s: S, reg_id: u32) -> bool;

pub uninterp spec fn IsImplementedCppcReg(s: S, reg_id: u32) -> bool;

pub uninterp spec fn ResultEqual(result: sbiret, code: i64) -> bool;

pub uninterp spec fn CppcReg(s: S, reg_id: u32) -> u64;

} // verus!
