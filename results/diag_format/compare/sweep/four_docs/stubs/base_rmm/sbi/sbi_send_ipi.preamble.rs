use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type HartId = u64;

pub type SbiErrorCode = i64;

pub struct SbiCommandReturnCode {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub hart_mask_base: u64,
    pub hart_mask: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn HartIdsFromMask(hart_mask_base: u64, hart_mask: u64) -> Set<HartId>;

pub open spec fn IsHartEnabledByPlatform(h: HartId) -> bool;

pub open spec fn IsHartAvailableToSupervisor(h: HartId) -> bool;

pub open spec fn ResultEqual(result: SbiCommandReturnCode, code: i64) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason() -> bool;

pub open spec fn SupervisorSoftwareInterruptPending(h: HartId) -> bool;

} // verus!
