use vstd::prelude::*;

verus! {

pub type SbiErrorCode = i64;
pub type HartId = u64;
pub type UInt64 = u64;

pub struct S {
    pub hart_mask_val: UInt64,
    pub hart_mask_base_val: UInt64,
}

pub spec const SBI_SUCCESS: SbiErrorCode = 0;
pub spec const SBI_ERR_FAILED: SbiErrorCode = -1;
pub spec const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub open spec fn hart_mask(s: S) -> UInt64;

pub open spec fn hart_mask_base(s: S) -> UInt64;

pub open spec fn IsTargetedHart(mask: UInt64, mask_base: UInt64, h: HartId) -> bool;

pub open spec fn IsHartEnabledByPlatform(h: HartId) -> bool;

pub open spec fn IsHartAvailableToSupervisor(h: HartId) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(mask: UInt64, mask_base: UInt64) -> bool;

pub open spec fn FenceIIpiSent(h: HartId) -> bool;

} // verus!
