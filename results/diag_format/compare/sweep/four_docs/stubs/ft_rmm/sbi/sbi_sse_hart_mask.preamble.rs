use vstd::prelude::*;
verus! {

pub type SbiErrorCode = i64;

pub type HartId = u64;

pub struct S {
    pub sse_masked: Map<HartId, bool>,
    pub failed: bool,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_ALREADY_STOPPED: SbiErrorCode = -8;

pub open spec fn CallingHart() -> HartId;

pub open spec fn HartSseMasked(s: S, hart: HartId) -> bool;

pub open spec fn RequestFailedForOtherReason(s: S) -> bool;

pub open spec fn ResultEqual(error: SbiErrorCode, code: SbiErrorCode) -> bool;

} // verus!
