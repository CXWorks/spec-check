use vstd::prelude::*;

verus! {

// Note: the function calls EventInjected and EventAt with two different
// argument counts. Verus/Rust has no overloading, so a single declaration
// cannot match both. Each one is declared here with its larger arity:
// EventInjected takes 2 arguments and EventAt takes 3.

pub type SbiReturnCode = i64;
pub type EventId = u32;
pub type HartId = u64;

pub const SBI_SUCCESS: SbiReturnCode = 0;

pub struct EventState {
    pub injected: bool,
}

pub struct S {
    pub event_id: EventId,
    pub hart_id: HartId,
}

pub open spec fn ResultEqual(a: SbiReturnCode, b: SbiReturnCode) -> bool;

pub open spec fn EventInjectAllowedByAttribute(event_id: EventId) -> bool;

pub open spec fn IsLocalEvent(event_id: EventId) -> bool;

pub open spec fn IsGlobalEvent(event_id: EventId) -> bool;

pub open spec fn EventInjected(event_id: EventId, hart_id: HartId) -> bool;

pub open spec fn InSseHandler(s: S) -> bool;

pub open spec fn EventReady(event_id: EventId) -> bool;

pub open spec fn EventPriority(event_id: EventId) -> int;

pub open spec fn CurrentEvent(s: S) -> EventId;

pub open spec fn EventHandledImmediately(event_id: EventId) -> bool;

pub open spec fn EventRunsAfterCompletion(event_id: EventId, current: EventId) -> bool;

pub open spec fn EventAt(s: S, event_id: EventId, hart_id: HartId) -> EventState;

} // verus!
