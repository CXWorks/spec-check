use vstd::prelude::*;

verus! {

pub type SbiErrorCode = i64;
pub type ChannelIdType = u32;
pub type MessageIdType = u32;
pub type HartId = u64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub struct S {
    pub dummy: u64,
}

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn ChannelId(s: S) -> ChannelIdType;

pub open spec fn MessageId(s: S) -> MessageIdType;

pub open spec fn MessageDataLen(s: S) -> u64;

pub open spec fn MessageDataMaxLen(s: S, channel: ChannelIdType) -> u64;

pub open spec fn CallingHart() -> HartId;

pub open spec fn SharedMemorySize(s: S, hart: HartId) -> u64;

pub open spec fn SharedMemoryData(s: S, hart: HartId, offset: u64, len: u64) -> Seq<u8>;

pub open spec fn MessageTransmitted(s: S, channel: ChannelIdType, msg_id: MessageIdType, data: Seq<u8>) -> bool;

} // verus!
