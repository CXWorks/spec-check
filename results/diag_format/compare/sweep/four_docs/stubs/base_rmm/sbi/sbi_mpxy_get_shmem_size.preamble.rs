use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub type HartId = u64;

pub type ChannelId = u32;

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub struct S {
    pub shmem_size: UInt,
    pub num_harts: u64,
}

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn MpxyShmemSize() -> UInt;

pub open spec fn MpxyShmemSizeOnHart(hart: HartId) -> UInt;

pub open spec fn MpxyChannels() -> Set<ChannelId>;

pub open spec fn MsgDataMaxLen(chan: ChannelId) -> UInt;

} // verus!
