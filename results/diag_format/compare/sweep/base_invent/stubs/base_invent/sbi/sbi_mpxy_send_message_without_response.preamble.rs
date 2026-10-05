use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub const MSG_DATA_MAX_LEN: u64 = 4096;

pub struct MpxyChannel {
    pub message_data_len: u64,
}

pub struct S {
    pub mpxy_channels: Map<u64, MpxyChannel>,
    pub channel_id: u64,
    pub hart_shared_memory_size: u64,
}

} // verus!
