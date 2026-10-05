use vstd::prelude::*;
verus! {

pub type HartId = u64;

pub type SseEventState = u64;

pub type SbiError = i64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct SseEvent {
    pub event_id: u64,
    pub state: SseEventState,
    pub priority: u64,
}

pub struct SupervisorState {
    pub pc: u64,
    pub a6: u64,
    pub a7: u64,
    pub sstatus: u64,
}

pub struct S {
    pub events: Map<HartId, Seq<SseEvent>>,
    pub supervisor_states: Map<HartId, SupervisorState>,
}

pub const SBI_SUCCESS: SbiError = 0;

pub const REGISTERED: SseEventState = 1;

pub const ENABLED: SseEventState = 2;

pub const current_hart: HartId = 0;

pub open spec fn ResultEqual(result: sbiret, code: SbiError) -> bool;

pub open spec fn HasEventInState(s: S, hart: HartId) -> bool;

pub open spec fn IsOneShot(e: SseEvent) -> bool;

pub open spec fn HighestPriorityRunningEvent(s: S, hart: HartId) -> SseEvent;

pub open spec fn ResumeSupervisorState(s: S, hart: HartId) -> SupervisorState;

} // verus!
