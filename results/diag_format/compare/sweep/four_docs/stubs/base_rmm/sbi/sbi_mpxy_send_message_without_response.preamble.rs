use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type SbiErrorCode = i64;
pub type HartId = u64;
pub type ChannelId = u64;
pub type MessageId = u64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub struct S {
    pub message_data_len: UInt64,
    pub channel_id: ChannelId,
    pub calling_hart: HartId,
    pub message_id: MessageId,
}

pub open spec fn MsgDataMaxLen(channel_id: ChannelId) -> UInt64;

pub open spec fn SharedMemorySize(hart: HartId) -> UInt64;

pub open spec fn SharedMemory(hart: HartId) -> Seq<u8>;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn MessageTransmitted(channel_id: ChannelId, message_id: MessageId, data: Seq<u8>) -> bool;

} // verus!
