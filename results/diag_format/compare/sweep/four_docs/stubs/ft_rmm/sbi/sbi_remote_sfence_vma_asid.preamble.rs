use vstd::prelude::*;

verus! {

pub type long = i64;
pub type unsigned = u64;
pub type HartId = u64;

pub struct S {
    pub harts_enabled: Set<HartId>,
    pub harts_available: Set<HartId>,
    pub ipi_sent: Set<HartId>,
    pub failed: bool,
}

pub spec const SBI_SUCCESS: i64 = 0;
pub spec const SBI_ERR_FAILED: i64 = -1;
pub spec const SBI_ERR_INVALID_PARAM: i64 = -3;
pub spec const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub open spec fn IsValidAddressRange(s: S, start_addr: u64, size: u64) -> bool;

pub open spec fn IsValidAsid(s: S, asid: u64) -> bool;

pub open spec fn ResultEqual(error: i64, code: i64) -> bool;

pub open spec fn HartsFromMask(s: S, hart_mask: u64, hart_mask_base: u64) -> Set<HartId>;

pub open spec fn IsHartEnabledByPlatform(s: S, hartid: HartId) -> bool;

pub open spec fn IsHartAvailableToSupervisor(s: S, hartid: HartId) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(s: S) -> bool;

pub open spec fn IpiSent(s: S, hartid: HartId) -> bool;

pub open spec fn SfenceVmaRequested(s: S, hartid: HartId, start_addr: u64, size: u64, asid: u64) -> bool;

} // verus!
