use vstd::prelude::*;

verus! {

pub type uint32_t = u32;

pub type UInt64 = u64;

pub type unsigned = u64;

pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;

pub type HartId = u64;

pub struct S {
    pub dummy: u64,
}

pub open spec fn MessageResponseReceived(s: S, channel_id: uint32_t) -> bool;

pub open spec fn ResultEqual(a: SbiCommandReturnCode, b: SbiCommandReturnCode) -> bool;

pub open spec fn calling_hart(s: S) -> HartId;

pub open spec fn SharedMemoryAt(s: S, hart: HartId, offset: u64) -> u64;

pub open spec fn MessageResponseData(s: S, channel_id: uint32_t) -> u64;

pub open spec fn MessageResponseDataLength(s: S, channel_id: uint32_t) -> UInt64;

} // verus!
