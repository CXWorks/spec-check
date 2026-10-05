use vstd::prelude::*;
verus! {

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_ALREADY_STARTED: SbiErrorCode = -7;

pub struct S {
    pub sse_hart_unmasked: bool,
    pub sse_unspecified_failure: bool,
}

pub open spec fn SseHartUnmasked(s: S) -> bool;

pub open spec fn SseUnspecifiedFailure(s: S) -> bool;

pub open spec fn ResultEqual(result: SbiErrorCode, expected: SbiErrorCode) -> bool;

} // verus!
