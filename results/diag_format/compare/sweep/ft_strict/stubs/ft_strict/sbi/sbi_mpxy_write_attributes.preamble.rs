use vstd::prelude::*;
verus! {

pub type uint32_t = u32;
pub type UInt32 = u32;
pub type SbiCommandReturnCode = i64;

pub spec const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub spec const SBI_ERR_FAILED: SbiCommandReturnCode = -1;

pub struct S {
    pub dummy: u64,
}

pub open spec fn ChannelAttribute(s: S, channel_id: uint32_t, attribute_id: int) -> u32;

pub open spec fn SharedMemoryWord(s: S, offset: int) -> u32;

} // verus!
