use vstd::prelude::*;
verus! {

pub type SbiErrorCode = i64;

pub struct S {
    pub sse_events_masked: bool,
    pub hart_ready: bool,
    pub failed: bool,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_ALREADY_STOPPED: SbiErrorCode = -8;

pub open spec fn SseEventsMasked(s: S) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(s: S) -> bool;

pub open spec fn SseHartReadyToReceiveEvents(s: S) -> bool;

pub open spec fn ResultEqual(result: SbiErrorCode, code: SbiErrorCode) -> bool;

} // verus!
