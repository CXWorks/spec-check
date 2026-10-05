use vstd::prelude::*;

verus! {

pub type long = i64;
pub type UInt = u64;
pub type HartId = u64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: long = 0;
pub const SBI_ERR_FAILED: long = -1;
pub const SBI_ERR_INVALID_PARAM: long = -3;
pub const SBI_ERR_INVALID_ADDRESS: long = -5;

pub open spec fn IsValidAddressRange(start_addr: UInt, size: UInt) -> bool;

pub open spec fn IsValidAsid(asid: UInt) -> bool;

pub open spec fn ResultEqual(error: long, code: long) -> bool;

pub open spec fn HartsFromMask(hart_mask: UInt, hart_mask_base: UInt) -> Set<HartId>;

pub open spec fn IsHartEnabledByPlatform(hartid: HartId) -> bool;

pub open spec fn IsHartAvailableToSupervisor(hartid: HartId) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason() -> bool;

pub open spec fn IpiSent(hartid: HartId) -> bool;

pub open spec fn SfenceVmaRequested(hartid: HartId, start_addr: UInt, size: UInt, asid: UInt) -> bool;

} // verus!
