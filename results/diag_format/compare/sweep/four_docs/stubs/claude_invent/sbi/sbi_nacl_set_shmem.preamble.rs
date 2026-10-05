use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;

pub type HartId = int;

pub struct S {
    pub calling_hart: int,
}

pub uninterp spec fn SbiShmemSatisfiesRequirements(s: S, addr: int, size: nat) -> bool;

pub uninterp spec fn CallingHart(s: S) -> HartId;

pub uninterp spec fn NaclShmemEnabled(s: S, hart: HartId) -> bool;

pub uninterp spec fn NaclShmemBase(s: S, hart: HartId) -> int;

} // verus!
