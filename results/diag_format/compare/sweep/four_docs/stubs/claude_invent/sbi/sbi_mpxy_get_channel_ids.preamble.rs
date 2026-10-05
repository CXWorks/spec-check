use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SbiError = i64;

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_FAILED: SbiError = -1;
pub const SBI_ERR_INVALID_PARAM: SbiError = -3;
pub const SBI_ERR_DENIED: SbiError = -4;
pub const SBI_ERR_NO_SHMEM: SbiError = -9;

pub struct S {
    pub dummy: int,
}

pub open spec fn MpxyShmemIsSetUp(s: S) -> bool;
pub open spec fn MpxyStartIndexValid(s: S, start_index: int) -> bool;
pub open spec fn MpxyGetChannelIdsAllowed(s: S) -> bool;
pub open spec fn MpxyChannelCount(s: S) -> u32;
pub open spec fn MpxyShmemReadU32(s: S, offset: int) -> u32;
pub open spec fn MpxyShmemChannelIdSlot(s: S, i: int) -> u32;
pub open spec fn MpxyChannelIdAt(s: S, i: int) -> u32;

} // verus!
