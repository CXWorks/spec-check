use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsValidHartId(s: S, hartid: UInt64) -> bool;

pub open spec fn ResultEqual(error: SbiErrorCode, expected: SbiErrorCode) -> bool;

pub open spec fn IsHsmStateId(value: UInt64) -> bool;

pub open spec fn ValueIsHsmStateOfHartDuringCall(s: S, hartid: UInt64, value: UInt64) -> bool;

} // verus!
