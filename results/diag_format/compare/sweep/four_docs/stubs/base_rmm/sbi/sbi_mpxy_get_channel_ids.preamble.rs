use vstd::prelude::*;
verus! {

pub type HartId = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct CmdInput {
    pub start_index: u32,
}

pub struct S {
    pub cmd_input: CmdInput,
    pub calling_hart: HartId,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_DENIED: i64 = -4;
pub const SBI_ERR_NO_SHMEM: i64 = -9;

pub const N: u32 = 16;

pub open spec fn IsValidChannelIdStartIndex(start_index: u32) -> bool;

pub open spec fn ResultEqual(result: sbiret, code: i64) -> bool;

pub open spec fn IsMpxyShmemSetUp(hart: HartId) -> bool;

pub open spec fn IsMpxyShmemDisabled(hart: HartId) -> bool;

pub open spec fn IsGetChannelIdsAllowed(hart: HartId) -> bool;

pub open spec fn OtherUnspecifiedError() -> bool;

pub open spec fn MpxyShmemWord(hart: HartId, offset: int) -> u32;

pub open spec fn RemainingChannelIdCount(start_index: u32, n: u32) -> u32;

pub open spec fn AccessibleChannelIds() -> Seq<u32>;

} // verus!
