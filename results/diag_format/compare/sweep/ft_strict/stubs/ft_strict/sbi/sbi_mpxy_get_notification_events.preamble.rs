use vstd::prelude::*;
verus! {

pub type uint32_t = u32;

pub struct SbiReturn(pub i64);

pub struct S {
    pub dummy: int,
}

pub open spec fn EventsStateEnabled(s: S, channel_id: uint32_t) -> bool;

pub open spec fn ChannelCapabilityEventsStateSet(s: S, channel_id: uint32_t) -> bool;

pub open spec fn ShmemWord32(s: S, channel_id: uint32_t, offset: u32) -> u32;

pub open spec fn EventsRemaining(s: S, channel_id: uint32_t) -> u32;

pub open spec fn EventsReturned(s: S, channel_id: uint32_t) -> u32;

pub open spec fn EventsLost(s: S, channel_id: uint32_t) -> u32;

pub open spec fn IsReservedField(s: S, channel_id: uint32_t, offset: u32) -> bool;

pub open spec fn NotificationEventsDataStartsAt(s: S, channel_id: uint32_t, offset: u32) -> bool;

} // verus!
