use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub type HartId = u64;

pub struct S {
    pub current_hart_id: HartId,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_ALREADY_STOPPED: SbiErrorCode = -8;

pub open spec fn current_hart(s: S) -> HartId;

pub open spec fn SseEventsMasked(s: S, hart: HartId) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(s: S, hart: HartId) -> bool;

pub open spec fn SseHartReadyToReceiveEvents(s: S, hart: HartId) -> bool;

pub open spec fn ResultEqual(result: SbiErrorCode, expected: SbiErrorCode) -> bool;

} // verus!
