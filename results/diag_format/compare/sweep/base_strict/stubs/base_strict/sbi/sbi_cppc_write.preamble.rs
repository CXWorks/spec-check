use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SbiError = i64;

pub struct S {
    pub dummy: int,
}

pub spec const SBI_SUCCESS: SbiError = 0;
pub spec const SBI_ERR_NOT_SUPPORTED: SbiError = (-2int) as i64;
pub spec const SBI_ERR_INVALID_PARAM: SbiError = (-3int) as i64;

pub uninterp spec fn CppcRegIsReserved(cppc_reg_id: UInt32) -> bool;
pub uninterp spec fn CppcRegIsImplemented(cppc_reg_id: UInt32) -> bool;
pub uninterp spec fn ResultEqual(a: SbiError, b: SbiError) -> bool;
pub uninterp spec fn CppcRegValue(cppc_reg_id: UInt32) -> UInt64;

} // verus!
