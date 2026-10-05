use vstd::prelude::*;

verus! {

pub type Hart = u64;

pub type EventId = u32;

pub type EventStateT = u32;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub spec const event_id: EventId = 0x00010000u32;

pub spec const UNUSED: EventStateT = 0u32;

pub spec const REGISTERED: EventStateT = 1u32;

pub spec const ENABLED: EventStateT = 2u32;

pub spec const RUNNING: EventStateT = 3u32;

pub open spec fn EventState(s: S, eid: EventId, h: Hart) -> EventStateT;

pub open spec fn CallingHart() -> Hart;

pub open spec fn ResultIsError(r: sbiret) -> bool;

pub open spec fn ResultIsSuccess(r: sbiret) -> bool;

pub open spec fn IsLocalEvent(eid: EventId) -> bool;

pub open spec fn IsGlobalEvent(eid: EventId) -> bool;

pub open spec fn HasRegisteredHandler(s: S, eid: EventId, h: Hart) -> bool;

} // verus!
