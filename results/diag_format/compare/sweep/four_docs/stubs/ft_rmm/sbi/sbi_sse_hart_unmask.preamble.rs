use vstd::prelude::*;
verus! {

pub type SbiErrorCode = i64;
pub type HartId = u64;

pub struct S {
    pub calling_hart: HartId,
    pub masked: Map<HartId, bool>,
    pub failed: bool,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_ALREADY_STARTED: SbiErrorCode = -7;

pub open spec fn SseEventsMaskedOnHart(s: S, hart: HartId) -> bool;
pub open spec fn CallingHart(s: S) -> HartId;
pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;
pub open spec fn RequestFailedForUnspecifiedReason(s: S) -> bool;

} // verus!
