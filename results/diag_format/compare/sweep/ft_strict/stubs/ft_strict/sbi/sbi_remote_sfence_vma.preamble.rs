use vstd::prelude::*;
verus! {

pub type unsigned_long = u64;

pub type HartId = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;

pub const result: bool = true;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsValidAddressRange(s: S, start_addr: unsigned_long, size: unsigned_long) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsTargetedHart(hart_mask: unsigned_long, hart_mask_base: unsigned_long, h: HartId) -> bool;

pub open spec fn IpiSent(h: HartId) -> bool;

pub open spec fn SfenceVmaExecuted(h: HartId, start: unsigned_long, end: int) -> bool;

pub open spec fn AddressTranslationCache(s: S) -> bool;

} // verus!
