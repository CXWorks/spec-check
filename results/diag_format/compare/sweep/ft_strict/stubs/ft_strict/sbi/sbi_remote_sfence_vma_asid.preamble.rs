use vstd::prelude::*;
verus! {

pub type long = i64;
pub type unsigned_long = u64;
pub type HartId = u64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub open spec fn IsValidAddressRange(s: S, start_addr: u64, size: u64) -> bool;

pub open spec fn IsValidAsid(s: S, asid: u64) -> bool;

pub open spec fn ResultEqual(result: i64, code: i64) -> bool;

pub open spec fn IsTargetedHart(s: S, hart_mask: u64, hart_mask_base: u64, h: HartId) -> bool;

pub open spec fn IsHartEnabledByPlatform(s: S, h: HartId) -> bool;

pub open spec fn IsHartAvailableToSupervisor(s: S, h: HartId) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(s: S, hart_mask: u64, hart_mask_base: u64, start_addr: u64, size: u64, asid: u64) -> bool;

pub open spec fn IpiSentToHart(s: S, h: HartId) -> bool;

pub open spec fn SfenceVmaAsidRequested(s: S, h: HartId, start: u64, end: int, asid: u64) -> bool;

} // verus!
