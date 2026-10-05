use vstd::prelude::*;
verus! {

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERROR_INVALID_RESET_TYPE: SbiErrorCode = -3;
pub const SBI_ERROR_INVALID_RESET_REASON: SbiErrorCode = -4;
pub const SBI_ERROR_INVALID_FID: SbiErrorCode = -5;

pub struct S {
    pub reset_type_reg: u32,
    pub reset_reason_reg: u32,
}

pub open spec fn reset_type(s: S) -> u32;

pub open spec fn SystemIsShutDown() -> bool;

pub open spec fn SystemIsColdRebooted() -> bool;

pub open spec fn SystemIsWarmRebooted() -> bool;

} // verus!
