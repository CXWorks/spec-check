use vstd::prelude::*;
verus! {

pub type uint32_t = u32;

pub type int64_t = i64;

pub type SseEventState = u32;

pub const SBI_SSE_EVENT_UNUSED: SseEventState = 0;
pub const SBI_SSE_EVENT_REGISTERED: SseEventState = 1;
pub const SBI_SSE_EVENT_ENABLED: SseEventState = 2;
pub const SBI_SSE_EVENT_RUNNING: SseEventState = 3;

pub const SBI_SSE_SUCCESS: int64_t = 0;
pub const SBI_SSE_ERR_FAILED: int64_t = -1;
pub const SBI_SSE_ERR_INVALID_STATE: int64_t = -2;

#[allow(non_upper_case_globals)]
pub spec const event_id: uint32_t = 0;

pub struct sbiret {
    pub code: int64_t,
    pub value: int64_t,
}

pub struct SseEvent {
    pub state: SseEventState,
}

pub struct S {
    pub sse_events: Seq<SseEvent>,
}

pub open spec fn event_enabled(s: S, event_id: uint32_t) -> bool;

pub open spec fn event_registered(s: S, event_id: uint32_t) -> bool;

} // verus!
