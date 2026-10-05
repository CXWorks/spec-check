use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt = u64;
pub type SbiErrorCode = i64;
pub type HartId = u64;

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub open spec fn MsgDataMaxLen(s: S, channel_id: UInt32) -> UInt;

pub open spec fn SharedMemorySize(s: S, hart: HartId) -> UInt;

pub open spec fn CallingHart() -> HartId;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn SharedMemoryData(s: S, hart: HartId, offset: UInt, len: UInt) -> Seq<u8>;

pub open spec fn MessageTransmitted(s: S, channel_id: UInt32, message_id: UInt32, data: Seq<u8>) -> bool;

} // verus!
