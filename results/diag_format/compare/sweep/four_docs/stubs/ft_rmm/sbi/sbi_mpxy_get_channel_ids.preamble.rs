use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type HartId = u64;
pub type sbiret = i64;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: sbiret = 0;
pub const SBI_ERR_FAILED: sbiret = -1;
pub const SBI_ERR_INVALID_PARAM: sbiret = -3;
pub const SBI_ERR_DENIED: sbiret = -4;
pub const SBI_ERR_NO_SHMEM: sbiret = -9;

pub open spec fn IsValidChannelIdStartIndex(s: S, start_index: UInt32) -> bool;
pub open spec fn ResultEqual(a: sbiret, b: sbiret) -> bool;
pub open spec fn calling_hart(s: S) -> HartId;
pub open spec fn IsMpxyShmemSetUp(s: S, hart: HartId) -> bool;
pub open spec fn IsMpxyShmemDisabled(s: S, hart: HartId) -> bool;
pub open spec fn IsGetChannelIdsAllowed(s: S, hart: HartId) -> bool;
pub open spec fn OtherUnspecifiedError(s: S) -> bool;
pub open spec fn MpxyShmemWord(s: S, hart: HartId, offset: int) -> u32;
pub open spec fn RemainingChannelIdCount(s: S, start_index: UInt32, returned: int) -> u32;
pub open spec fn AccessibleChannelIds(s: S) -> Seq<u32>;

} // verus!
