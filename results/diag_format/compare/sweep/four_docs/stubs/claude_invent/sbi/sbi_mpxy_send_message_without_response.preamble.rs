use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int64 = i64;

pub struct SbiRet {
    pub error: Int64,
    pub value: UInt64,
}

pub struct S {
    pub mpxy_shmem_size: UInt64,
    pub state_id: nat,
}

pub const SBI_SUCCESS: Int64 = 0;
pub const SBI_ERR_INVALID_PARAM: Int64 = -3;

pub open spec fn MpxyMsgDataMaxLen(s: S, channel_id: UInt32) -> UInt64;

pub open spec fn MpxyShmemSize(s: S) -> UInt64;

pub open spec fn MpxyPostedMessageSent(old_s: S, new_s: S, channel_id: UInt32, message_id: UInt32, message_data_len: UInt64) -> bool;

} // verus!
