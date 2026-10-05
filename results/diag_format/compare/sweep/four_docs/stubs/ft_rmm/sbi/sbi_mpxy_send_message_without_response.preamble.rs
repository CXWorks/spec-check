use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub type UInt = nat;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

#[allow(non_upper_case_globals)]
pub const calling_hart: UInt64 = 0;

pub struct MpxyChannelState {
    pub transmitted_messages: nat,
}

pub struct S {
    pub dummy: nat,
}

pub open spec fn MsgDataMaxLen(s: S, channel_id: UInt32) -> UInt;

pub open spec fn SharedMemorySize(s: S, hart: UInt64) -> UInt;

pub open spec fn SharedMemory(s: S, hart: UInt64) -> Seq<u8>;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn ResultNotEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn MessageTransmitted(s: S, channel_id: UInt32, message_id: UInt32, data: Seq<u8>) -> bool;

pub open spec fn MpxyChannel(s: S, channel_id: UInt32) -> MpxyChannelState;

} // verus!
