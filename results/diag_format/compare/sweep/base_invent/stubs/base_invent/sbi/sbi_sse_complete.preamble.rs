use vstd::prelude::*;

verus! {

pub type sbiret = i64;

pub spec const SBI_SUCCESS: sbiret = 0;

pub enum SseEventState {
    UNUSED,
    REGISTERED,
    ENABLED,
    RUNNING,
}

pub enum SseEventConfig {
    ONE_SHOT,
    NORMAL,
}

pub struct SseEvent {
    pub hart_id: u64,
    pub state: SseEventState,
    pub priority: u64,
    pub config: SseEventConfig,
}

pub struct S {
    pub hart_id: u64,
    pub sse_events: Seq<SseEvent>,
}

} // verus!
