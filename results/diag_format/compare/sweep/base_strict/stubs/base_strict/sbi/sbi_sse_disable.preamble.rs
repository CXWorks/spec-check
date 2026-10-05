use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub type Hart = u64;
pub type EventId = u32;
pub type SseEventState = u32;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub event_id: EventId,
    pub hart: Hart,
}

pub const ENABLED: SseEventState = 2;
pub const REGISTERED: SseEventState = 1;

pub open spec fn event_id(s: S) -> EventId;

pub open spec fn CallingHart() -> Hart;

pub open spec fn EventState(id: EventId, h: Hart) -> SseEventState;

pub open spec fn ResultIsError(r: sbiret) -> bool;

pub open spec fn ResultIsSuccess(r: sbiret) -> bool;

pub open spec fn IsLocalEvent(id: EventId) -> bool;

pub open spec fn IsGlobalEvent(id: EventId) -> bool;

} // verus!
