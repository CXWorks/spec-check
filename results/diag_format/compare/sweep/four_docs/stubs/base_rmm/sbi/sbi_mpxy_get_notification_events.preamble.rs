use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type UInt32 = u32;

pub struct sbiret {
    pub error: i64,
    pub value: (),
}

pub struct S {
    pub shmem: Seq<u8>,
}

#[allow(non_upper_case_globals)]
pub spec const channel_id: u64 = 0;

pub open spec fn ChannelSupportsEventsState(s: S, channel: u64) -> bool;

pub open spec fn EventsStateEnabled(s: S, channel: u64) -> bool;

pub open spec fn ShmemWord32(s: S, channel: u64, offset: u64) -> u32;

pub open spec fn EventsRemaining(channel: u64) -> u32;

pub open spec fn EventsReturned(channel: u64) -> u32;

pub open spec fn EventsLost(channel: u64) -> u32;

pub open spec fn ShmemNotificationData(s: S, channel: u64, offset: u64) -> Seq<u8>;

pub open spec fn ProtocolNotificationEvents(channel: u64) -> Seq<u8>;

} // verus!
