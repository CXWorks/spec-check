use vstd::prelude::*;
verus! {

pub type HartId = u64;

pub struct S {
    pub dummy: u64,
}

pub open spec fn ChannelAttribute(channel_id: u32, attribute_id: int) -> u32;

pub open spec fn CallingHart() -> HartId;

pub open spec fn SharedMemWord(hart: HartId, offset: int) -> u32;

} // verus!
