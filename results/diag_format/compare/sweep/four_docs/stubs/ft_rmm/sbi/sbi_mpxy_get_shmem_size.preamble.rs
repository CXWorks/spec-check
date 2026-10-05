use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_DENIED: SbiErrorCode = -4;
pub const SBI_ERR_INVALID_ADDRESS: SbiErrorCode = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: SbiErrorCode = -6;
pub const SBI_ERR_ALREADY_STARTED: SbiErrorCode = -7;
pub const SBI_ERR_ALREADY_STOPPED: SbiErrorCode = -8;
pub const SBI_ERR_NO_SHMEM: SbiErrorCode = -9;

pub struct S {
    pub shmem_size: UInt,
    pub hart_shmem_size: Map<int, UInt>,
    pub channels: Set<UInt>,
}

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn MpxyShmemSize() -> UInt;

pub open spec fn MpxyShmemSizeOnHart(hart: int) -> UInt;

pub open spec fn MpxyChannels() -> Set<UInt>;

pub open spec fn MsgDataMaxLen(chan: UInt) -> UInt;

} // verus!
