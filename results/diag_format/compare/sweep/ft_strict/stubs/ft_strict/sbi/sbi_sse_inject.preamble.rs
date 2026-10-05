use vstd::prelude::*;
verus! {

pub type uint32_t = u32;
pub type unsigned_long = u64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub const SBI_SUCCESS: sbiret = sbiret { error: 0, value: 0 };

pub struct SseEvent {
    pub injected: bool,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn EventAttributeAllowsInjection(s: S, event_id: uint32_t) -> bool;
pub open spec fn IsLocalEvent(s: S, event_id: uint32_t) -> bool;
pub open spec fn IsGlobalEvent(s: S, event_id: uint32_t) -> bool;
pub open spec fn EventInjectedOnHart(s: S, event_id: uint32_t, hart_id: unsigned_long) -> bool;
pub open spec fn EventInjected(s: S, event_id: uint32_t) -> bool;
pub open spec fn InSseEventHandler(s: S) -> bool;
pub open spec fn EventReadyToRun(s: S, event_id: uint32_t) -> bool;
pub open spec fn EventPriority(s: S, event_id: uint32_t) -> int;
pub open spec fn RunningSseEvent(s: S) -> uint32_t;
pub open spec fn EventHandledImmediately(s: S, event_id: uint32_t) -> bool;
pub open spec fn EventRunsAfterCompletionOf(s: S, event_id: uint32_t, running: uint32_t) -> bool;
pub open spec fn SseEventAt(s: S, event_id: uint32_t, hart_id: unsigned_long) -> SseEvent;

} // verus!
