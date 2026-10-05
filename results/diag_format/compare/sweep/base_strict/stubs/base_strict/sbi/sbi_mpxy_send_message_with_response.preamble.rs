use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub type ChannelId = u32;

pub type MessageId = u32;

pub type HartId = u64;

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub struct S {
    pub channel: ChannelId,
    pub message: MessageId,
    pub hart: HartId,
}

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn channel_id(s: S) -> ChannelId;

pub open spec fn message_id(s: S) -> MessageId;

pub open spec fn CallingHart(s: S) -> HartId;

pub open spec fn MessageResponseReceived(s: S, ch: ChannelId, msg: MessageId) -> bool;

pub open spec fn SharedMemResponseWritten(s: S, hart: HartId, offset: UInt, value: UInt) -> bool;

pub open spec fn MessageResponseDataLen(s: S, ch: ChannelId, msg: MessageId) -> UInt;

pub open spec fn ChannelCapabilitySendWithResponseBit(s: S, ch: ChannelId) -> UInt;

} // verus!
