use vstd::prelude::*;

verus! {

pub type SbiErrorCode = i64;
pub type HartId = u64;

pub struct S {
    pub hart_mask: u64,
    pub hart_mask_base: u64,
    pub start_addr: int,
    pub size: int,
}

pub spec const SBI_SUCCESS: SbiErrorCode = 0;
pub spec const SBI_ERR_FAILED: SbiErrorCode = -1;
pub spec const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub spec const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub spec const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;

pub open spec fn IsFunctionImplemented(s: S, fid: u64) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsTargetHart(h: HartId, hart_mask: u64, hart_mask_base: u64) -> bool;

pub open spec fn HartSupportsHypervisorExtension(s: S, h: HartId) -> bool;

pub open spec fn IsValidAddressRange(s: S, start: int, size: int) -> bool;

pub open spec fn IsHartEnabledByPlatform(s: S, h: HartId) -> bool;

pub open spec fn IsHartAvailableToSupervisor(s: S, h: HartId) -> bool;

pub open spec fn RequestFailedForOtherReason(hart_mask: u64, hart_mask_base: u64, start: int, size: int) -> bool;

pub open spec fn IpiSentToHart(s: S, hart_mask: u64, hart_mask_base: u64, start: int, size: int) -> bool;

pub open spec fn HfenceGvmaExecuted(s: S, h: HartId, start: int, end: int) -> bool;

pub open spec fn GuestPhysicalTranslationState(s: S, h: HartId, start: int, end: int) -> bool;

} // verus!
