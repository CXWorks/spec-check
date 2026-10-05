use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub shmem: Map<UInt32, Seq<u8>>,
    pub events_state_supported: Map<UInt32, bool>,
    pub events_state_enabled: Map<UInt32, bool>,
    pub events_remaining: Map<UInt32, UInt32>,
    pub events_returned: Map<UInt32, UInt32>,
    pub events_lost: Map<UInt32, UInt32>,
    pub notification_events: Map<UInt32, Seq<u8>>,
}

pub open spec fn ChannelSupportsEventsState(s: S, channel_id: UInt32) -> bool;

pub open spec fn EventsStateEnabled(s: S, channel_id: UInt32) -> bool;

pub open spec fn ShmemWord32(s: S, channel_id: UInt32, offset: int) -> UInt32;

pub open spec fn EventsRemaining(s: S, channel_id: UInt32) -> UInt32;

pub open spec fn EventsReturned(s: S, channel_id: UInt32) -> UInt32;

pub open spec fn EventsLost(s: S, channel_id: UInt32) -> UInt32;

pub open spec fn ShmemNotificationData(s: S, channel_id: UInt32, offset: int) -> Seq<u8>;

pub open spec fn ProtocolNotificationEvents(s: S, channel_id: UInt32) -> Seq<u8>;

} // verus!
