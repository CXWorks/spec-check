use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type EventId = u32;

pub struct SbiRet {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: i64 = 0;

pub const SSE_STATE_UNUSED: u32 = 0;
pub const SSE_STATE_REGISTERED: u32 = 1;
pub const SSE_STATE_ENABLED: u32 = 2;
pub const SSE_STATE_RUNNING: u32 = 3;

pub open spec fn SseHartHasRunningEvent(s: S) -> bool;

pub open spec fn SseHighestPriorityRunningEvent(s: S) -> EventId;

pub open spec fn SseEventIsOneShot(s: S, ev: EventId) -> bool;

pub open spec fn SseEventState(s: S, ev: EventId) -> u32;

pub open spec fn SseInterruptedSupervisorStateResumed(old_s: S, new_s: S, ev: EventId) -> bool;

} // verus!
