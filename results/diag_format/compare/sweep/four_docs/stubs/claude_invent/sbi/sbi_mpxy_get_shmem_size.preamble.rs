use vstd::prelude::*;
verus! {

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;

pub struct S {
    pub mpxy_shmem_size: u64,
    pub mpxy_max_msg_data_max_len: u64,
}

pub open spec fn MpxyShmemSize(s: S) -> u64;

pub open spec fn MpxyMaxMsgDataMaxLenAllChannels(s: S) -> u64;

} // verus!
