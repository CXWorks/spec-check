use vstd::prelude::*;
verus! {

pub type SbiErrorCode = i64;
pub type HartId = u64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_ALREADY_STARTED: SbiErrorCode = -7;

pub struct S {
    pub dummy: u64,
}

pub open spec fn CurrentHartId(s: S) -> HartId;

pub open spec fn SseHartUnmasked(s: S, hart: HartId) -> bool;

} // verus!
