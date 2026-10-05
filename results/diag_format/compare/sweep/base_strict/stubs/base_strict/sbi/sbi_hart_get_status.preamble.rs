use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsValidHartId(hartid: UInt64) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsHsmStateId(value: UInt64) -> bool;

pub open spec fn ValueIsHsmStateOfHartDuringCall(hartid: UInt64, value: UInt64) -> bool;

} // verus!
