use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int64 = i64;

pub struct SbiRet {
    pub error: Int64,
    pub uvalue: UInt64,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: Int64 = 0;

pub open spec fn MpxyResponseReceived(s: S, channel_id: UInt32, message_id: UInt32, message_data_len: UInt64) -> bool;

pub open spec fn MpxyChannelCapabilitySendWithResponse(s: S, channel_id: UInt32) -> bool;

pub open spec fn MpxyResponseDataLen(s: S, channel_id: UInt32, message_id: UInt32, message_data_len: UInt64) -> UInt64;

pub open spec fn MpxyShmemResponseWritten(old_s: S, new_s: S, channel_id: UInt32, message_id: UInt32, message_data_len: UInt64, resp_len: UInt64) -> bool;

} // verus!
