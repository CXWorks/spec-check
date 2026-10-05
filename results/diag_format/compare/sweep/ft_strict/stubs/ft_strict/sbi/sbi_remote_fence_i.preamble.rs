use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type HartId = u64;

#[derive(PartialEq, Eq)]
pub enum SbiErrorCode {
    Success,
    Failed,
    InvalidParam,
}

pub struct S {
    pub fence_i_ipi_sent: bool,
}

pub spec const SBI_SUCCESS: SbiErrorCode = SbiErrorCode::Success;

pub spec const SBI_ERR_FAILED: SbiErrorCode = SbiErrorCode::Failed;

pub spec const SBI_ERR_INVALID_PARAM: SbiErrorCode = SbiErrorCode::InvalidParam;

pub open spec fn IsTargetedHart(hart_mask: UInt64, hart_mask_base: UInt64, h: HartId) -> bool;

pub open spec fn IsHartEnabledByPlatform(h: HartId) -> bool;

pub open spec fn IsHartAvailableToSupervisor(h: HartId) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(hart_mask: UInt64, hart_mask_base: UInt64) -> bool;

pub open spec fn FenceIIpiSent(s: S) -> bool;

} // verus!
