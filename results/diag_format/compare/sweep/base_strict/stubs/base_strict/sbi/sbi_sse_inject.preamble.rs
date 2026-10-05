use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type SbiCommandReturnCode = i64;

pub type EventId = u32;

pub type HartId = u64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_FAILED: SbiCommandReturnCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiCommandReturnCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;

pub struct SseEvent {
    pub injected: bool,
    pub priority: u32,
}

pub struct S {
    pub cmd_input_event_id: EventId,
    pub cmd_input_hart_id: HartId,
}

pub open spec fn EventAttributeAllowsInjection(s: S, event_id: EventId) -> bool;

pub open spec fn ResultEqual(a: SbiCommandReturnCode, b: SbiCommandReturnCode) -> bool;

pub open spec fn IsLocalEvent(s: S, event_id: EventId) -> bool;

pub open spec fn IsGlobalEvent(s: S, event_id: EventId) -> bool;

pub open spec fn EventInjectedOnHart(s: S, event_id: EventId, hart_id: HartId) -> bool;

pub open spec fn EventInjected(s: S, event_id: EventId) -> bool;

pub open spec fn InSseEventHandler(s: S) -> bool;

pub open spec fn EventReadyToRun(s: S, event_id: EventId) -> bool;

pub open spec fn EventPriority(s: S, event_id: EventId) -> int;

pub open spec fn RunningSseEvent(s: S) -> EventId;

pub open spec fn EventHandledImmediately(s: S, event_id: EventId) -> bool;

pub open spec fn EventRunsAfterCompletionOf(s: S, event_id: EventId, running: EventId) -> bool;

pub open spec fn SseEventAt(s: S, event_id: EventId, hart_id: HartId) -> SseEvent;

} // verus!
