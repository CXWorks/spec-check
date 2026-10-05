use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type HartId = u64;

pub type SbiErrorCode = i64;

pub struct S {
    pub dummy: u64,
}

pub open spec const SBI_SUCCESS: SbiErrorCode = 0;

pub open spec const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = (-5) as i64;

pub open spec fn IsValidAddressRange(start_addr: UInt, size: UInt) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsTargetedHart(hart_mask: UInt, hart_mask_base: UInt, h: HartId) -> bool;

pub open spec fn IpiSent(h: HartId) -> bool;

pub open spec fn SfenceVmaExecuted(h: HartId, start: UInt, end: int) -> bool;

pub open spec fn AddressTranslationCache(h: HartId) -> bool;

} // verus!
