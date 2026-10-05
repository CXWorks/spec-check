use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub hart_id: u64,
    pub channel: u64,
}

pub const SBI_SUCCESS: i64 = 0;

pub open spec fn ResultEqual(r: sbiret, code: i64) -> bool;

pub open spec fn MessageResponseReceived(channel: u64) -> bool;

pub open spec fn channel_id(s: S) -> u64;

pub open spec fn calling_hart(s: S) -> u64;

pub open spec fn SharedMemoryAt(hart: u64, offset: u64) -> Seq<u8>;

pub open spec fn MessageResponseData(channel: u64) -> Seq<u8>;

pub open spec fn MessageResponseDataLength(channel: u64) -> u64;

} // verus!
