use vstd::prelude::*;

verus! {

pub type SbiErrorCode = i64;

pub struct S {
    pub hart_id: u64,
    pub sse_masked: bool,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_ALREADY_STOPPED: SbiErrorCode = -8;

pub open spec fn HartSseMasked(s: S) -> bool;

pub open spec fn RequestFailedForOtherReason(s: S) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

} // verus!
