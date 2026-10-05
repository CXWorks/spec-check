use vstd::prelude::*;
verus! {

pub type SbiError = i64;

pub type HartId = u64;

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_FAILED: SbiError = -1;
pub const SBI_ERR_ALREADY_STOPPED: SbiError = -8;

pub struct S {
    pub dummy: int,
}

pub open spec fn SseHartMasked(s: S, hart: HartId) -> bool;

pub open spec fn CurrentHartId(s: S) -> HartId;

} // verus!
