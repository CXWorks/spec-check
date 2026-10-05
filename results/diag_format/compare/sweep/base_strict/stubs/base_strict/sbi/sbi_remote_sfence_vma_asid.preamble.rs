use vstd::prelude::*;

verus! {

pub type long = i64;

pub type unsigned_long = u64;

pub type HartId = u64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: long = 0;
pub const SBI_ERR_FAILED: long = -1;
pub const SBI_ERR_INVALID_PARAM: long = -3;
pub const SBI_ERR_INVALID_ADDRESS: long = -5;

pub open spec fn IsValidAddressRange(start_addr: unsigned_long, size: unsigned_long) -> bool;

pub open spec fn ResultEqual(result: long, code: long) -> bool;

pub open spec fn IsValidAsid(asid: unsigned_long) -> bool;

pub open spec fn IsTargetedHart(hart_mask: unsigned_long, hart_mask_base: unsigned_long, h: HartId) -> bool;

pub open spec fn IsHartEnabledByPlatform(h: HartId) -> bool;

pub open spec fn IsHartAvailableToSupervisor(h: HartId) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(hart_mask: unsigned_long, hart_mask_base: unsigned_long, start_addr: unsigned_long, size: unsigned_long, asid: unsigned_long) -> bool;

pub open spec fn IpiSentToHart(h: HartId) -> bool;

pub open spec fn SfenceVmaAsidRequested(h: HartId, start: unsigned_long, end: int, asid: unsigned_long) -> bool;

pub open spec fn AddressTranslationCache(h: HartId, start: unsigned_long, end: int, asid: unsigned_long) -> bool;

} // verus!
