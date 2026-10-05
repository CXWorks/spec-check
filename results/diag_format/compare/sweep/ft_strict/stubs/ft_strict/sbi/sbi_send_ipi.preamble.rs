use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub type HartId = u64;

pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub open spec fn IsTargetedHart(hart_mask: UInt, hart_mask_base: UInt, h: HartId) -> bool;

pub open spec fn IsHartEnabledByPlatform(h: HartId) -> bool;

pub open spec fn IsHartAvailableToSupervisor(h: HartId) -> bool;

pub open spec fn ResultEqual(error: SbiErrorCode, code: SbiErrorCode) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(s: S) -> bool;

pub open spec fn SupervisorSoftwareInterruptPending(s: S) -> bool;

} // verus!
