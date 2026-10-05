use vstd::prelude::*;
verus! {

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_ALREADY_STARTED: SbiErrorCode = -7;

pub type HartId = u64;

pub struct S {
    pub dummy: u64,
}

pub open spec fn CurrentHart() -> HartId;

pub open spec fn SseHartUnmasked(s: S, hart: HartId) -> bool;

pub open spec fn SseUnspecifiedFailure(s: S, hart: HartId) -> bool;

pub open spec fn ResultEqual(result: SbiErrorCode, expected: SbiErrorCode) -> bool;

} // verus!
