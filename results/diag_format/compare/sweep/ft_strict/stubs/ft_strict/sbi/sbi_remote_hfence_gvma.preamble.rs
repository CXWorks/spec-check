use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Address = u64;
pub type HartId = u64;
pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: u64,
}

pub spec const SBI_SUCCESS: SbiErrorCode = 0;
pub spec const SBI_ERR_FAILED: SbiErrorCode = -1;
pub spec const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub spec const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub spec const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;

pub spec const h: HartId = 0;

pub open spec fn IsFunctionImplemented(s: S, fid: UInt64) -> bool;
pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;
pub open spec fn IsTargetHart(s: S, h: HartId, hart_mask: UInt64, hart_mask_base: UInt64) -> bool;
pub open spec fn HartSupportsHypervisorExtension(s: S, h: HartId) -> bool;
pub open spec fn IsValidAddressRange(s: S, start_addr: Address, size: UInt64) -> bool;
pub open spec fn IsHartEnabledByPlatform(s: S, h: HartId) -> bool;
pub open spec fn IsHartAvailableToSupervisor(s: S, h: HartId) -> bool;
pub open spec fn RequestFailedForOtherReason(s: S, hart_mask: UInt64, hart_mask_base: UInt64, start_addr: Address, size: UInt64) -> bool;
pub open spec fn IpiSentToHart(s: S, h: HartId) -> bool;
pub open spec fn HfenceGvmaExecuted(s: S, h: HartId, start: int, end: int) -> bool;

} // verus!
