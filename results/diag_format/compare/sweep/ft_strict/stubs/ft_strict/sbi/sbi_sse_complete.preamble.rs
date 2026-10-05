use vstd::prelude::*;

verus! {

pub type SbiReturnCode = i64;

pub const SBI_SUCCESS: SbiReturnCode = 0;
pub const SBI_ERR_FAILED: SbiReturnCode = -1;

pub type HartId = u64;

pub type SseEventState = u8;

pub const UNUSED: SseEventState = 0;
pub const REGISTERED: SseEventState = 1;
pub const ENABLED: SseEventState = 2;
pub const RUNNING: SseEventState = 3;

pub struct SseEvent {
    pub event_id: u32,
    pub state: SseEventState,
    pub priority: u32,
    pub attr: u64,
}

pub struct SupervisorContext {
    pub pc: u64,
    pub a6: u64,
    pub a7: u64,
    pub sstatus: u64,
}

pub struct S {
    pub calling_hart: HartId,
    pub events: Map<(HartId, u32), SseEvent>,
    pub supervisor: Map<HartId, SupervisorContext>,
}

pub open spec fn CallingHart(s: S) -> HartId;

pub open spec fn PreExistsRunningEvent(s: S, hart: HartId) -> bool;

pub open spec fn HartSseStateUnchanged(s: S, hart: HartId) -> bool;

pub open spec fn PreHighestPriorityRunningEvent(s: S, hart: HartId) -> SseEvent;

pub open spec fn IsOneShot(e: SseEvent) -> bool;

pub open spec fn InterruptedSupervisorStateResumed(s: S, hart: HartId, e: SseEvent) -> bool;

pub open spec fn SupervisorState(s: S, hart: HartId) -> SupervisorContext;

} // verus!
