use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SbiError = i64;

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_NOT_SUPPORTED: SbiError = -2;
pub const SBI_ERR_INVALID_PARAM: SbiError = -3;

pub struct S {
    pub dummy: int,
}

pub open spec fn CppcRegIsReserved(s: S, cppc_reg_id: UInt32) -> bool;

pub open spec fn CppcRegIsImplemented(s: S, cppc_reg_id: UInt32) -> bool;

pub open spec fn CppcRegValue(s: S, cppc_reg_id: UInt32) -> UInt64;

pub open spec fn ResultEqual(result: SbiError, expected: SbiError) -> bool;

} // verus!
