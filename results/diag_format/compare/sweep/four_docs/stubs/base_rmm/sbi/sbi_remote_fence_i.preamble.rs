use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type SbiErrorCode = i64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub ipi_pending: Map<UInt64, bool>,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub open spec fn HartsFromMask(hart_mask: UInt64, hart_mask_base: UInt64) -> Set<UInt64>;

pub open spec fn IsHartEnabledByPlatform(hartid: UInt64) -> bool;

pub open spec fn IsHartAvailableToSupervisor(hartid: UInt64) -> bool;

pub open spec fn ResultEqual(result: sbiret, code: SbiErrorCode) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason() -> bool;

pub open spec fn IpiSentTo(hartid: UInt64) -> bool;

} // verus!
