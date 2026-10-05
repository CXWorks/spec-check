use vstd::prelude::*;
verus! {

pub type SbiErrorCode = i64;

pub type HartId = u64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_ALREADY_STARTED: SbiErrorCode = -7;

pub open spec fn SseEventsMaskedOnHart(s: S, hart: HartId) -> bool;

pub open spec fn CallingHart() -> HartId;

pub open spec fn RequestFailedForUnspecifiedReason() -> bool;

pub open spec fn ResultEqual(result: SbiErrorCode, code: SbiErrorCode) -> bool;

} // verus!
