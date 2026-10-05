use vstd::prelude::*;

verus! {

pub type UInt64 = u64;
pub type HartId = u64;
pub type EventId = u32;
pub type EventState = u32;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub spec const SBI_SUCCESS: sbiret = sbiret { error: 0, value: 0 };

pub spec const RUNNING: EventState = 3;
pub spec const REGISTERED: EventState = 1;
pub spec const ENABLED: EventState = 2;

pub spec const current_hart: HartId = 0;

pub struct Event {
    pub id: EventId,
    pub state: EventState,
}

pub struct S {
    pub events: Map<EventId, Event>,
    pub hart_events: Map<HartId, Set<EventId>>,
}

pub open spec fn HasEventInState(s: S, hart: HartId, state: EventState) -> bool;

pub open spec fn IsOneShot(s: S, e: Event) -> bool;

pub open spec fn HighestPriorityRunningEvent(s: S, hart: HartId) -> Event;

} // verus!
