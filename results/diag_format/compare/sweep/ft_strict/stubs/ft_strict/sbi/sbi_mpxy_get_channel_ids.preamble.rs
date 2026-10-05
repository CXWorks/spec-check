use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt = u64;
pub type HartId = u64;
pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_DENIED: SbiErrorCode = -4;
pub const SBI_ERR_NO_SHMEM: SbiErrorCode = -9;

pub struct S {
    pub dummy: u64,
}

pub open spec fn CallingHart(s: S) -> HartId;

pub open spec fn IsValidChannelIdStartIndex(s: S, start_index: UInt32) -> bool;

pub open spec fn MpxyShmemEnabled(s: S, hart: HartId) -> bool;

pub open spec fn GetChannelIdsAllowed(s: S, hart: HartId) -> bool;

pub open spec fn UnspecifiedFailure(s: S, hart: HartId) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn ShmemReturned(s: S, hart: HartId) -> UInt32;

pub open spec fn NumChannelIdsWritten(s: S, hart: HartId) -> UInt32;

pub open spec fn ShmemRemaining(s: S, hart: HartId) -> UInt32;

pub open spec fn RemainingCountsChannelIdsAfterReturned(s: S, hart: HartId, start_index: UInt32, remaining: UInt32) -> bool;

pub open spec fn ShmemChannelId(s: S, hart: HartId, i: UInt32) -> UInt32;

pub open spec fn AccessibleChannelId(s: S, hart: HartId, idx: int) -> UInt32;

pub open spec fn ShmemChannelIdArray(s: S, hart: HartId) -> Seq<UInt32>;

} // verus!
