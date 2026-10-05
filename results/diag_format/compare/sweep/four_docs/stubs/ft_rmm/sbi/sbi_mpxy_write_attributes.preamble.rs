use vstd::prelude::*;
verus! {

pub type uint32 = u32;

pub type uint64 = u64;

pub type HartId = u64;

pub type SbiReturnCode = i64;

pub const SBI_SUCCESS: SbiReturnCode = 0;

pub const SBI_ERR_FAILED: SbiReturnCode = -1;

pub struct S {
    pub dummy: u64,
}

pub open spec fn ChannelAttribute(s: S, channel_id: uint32, attribute_id: int) -> uint32;

pub open spec fn SharedMemWord(s: S, hart: HartId, offset: int) -> uint32;

pub open spec fn CallingHart(s: S) -> HartId;

} // verus!
