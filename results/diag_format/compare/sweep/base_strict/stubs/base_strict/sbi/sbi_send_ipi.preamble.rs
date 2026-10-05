use vstd::prelude::*;

verus! {

pub type HartId = u64;

pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: u64,
}

pub spec const SBI_SUCCESS: SbiErrorCode = 0;
pub spec const SBI_ERR_FAILED: SbiErrorCode = -1;
pub spec const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub spec const hart_mask: u64 = 1;
pub spec const hart_mask_base: u64 = 2;

pub open spec fn IsTargetedHart(mask: u64, mask_base: u64, h: HartId) -> bool;

pub open spec fn IsHartEnabledByPlatform(h: HartId) -> bool;

pub open spec fn IsHartAvailableToSupervisor(h: HartId) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason() -> bool;

pub open spec fn SupervisorSoftwareInterruptPending(h: HartId) -> bool;

} // verus!
