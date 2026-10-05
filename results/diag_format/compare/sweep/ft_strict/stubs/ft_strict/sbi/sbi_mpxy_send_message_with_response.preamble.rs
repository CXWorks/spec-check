use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt = u64;
pub type HartId = u64;

pub enum SbiErrorCode {
    SBI_SUCCESS,
    SBI_ERR_FAILED,
    SBI_ERR_NOT_SUPPORTED,
    SBI_ERR_INVALID_PARAM,
    SBI_ERR_DENIED,
    SBI_ERR_INVALID_ADDRESS,
    SBI_ERR_ALREADY_AVAILABLE,
}

pub use SbiErrorCode::*;

pub struct S {
    pub dummy: int,
}

pub open spec fn MessageResponseReceived(s: S, channel_id: UInt32, message_id: UInt32) -> bool;

pub open spec fn CallingHart(s: S) -> HartId;

pub open spec fn SharedMemResponseWritten(s: S, hart: HartId, offset: int, value: UInt) -> bool;

pub open spec fn MessageResponseDataLen(s: S, channel_id: UInt32, message_id: UInt32) -> UInt;

pub open spec fn ChannelCapabilitySendWithResponseBit(s: S, channel_id: UInt32) -> int;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

} // verus!
