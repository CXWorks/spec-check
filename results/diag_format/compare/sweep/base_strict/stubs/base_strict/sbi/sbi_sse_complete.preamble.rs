use vstd::prelude::*;

verus! {

pub type SbiReturnCode = i64;

pub type HartId = u64;

pub type SseEventId = u32;

pub type SseEventState = u8;

pub const SBI_SUCCESS: SbiReturnCode = 0;
pub const SBI_ERR_FAILED: SbiReturnCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiReturnCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiReturnCode = -3;
pub const SBI_ERR_DENIED: SbiReturnCode = -4;
pub const SBI_ERR_INVALID_ADDRESS: SbiReturnCode = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: SbiReturnCode = -6;
pub const SBI_ERR_ALREADY_STARTED: SbiReturnCode = -7;
pub const SBI_ERR_ALREADY_STOPPED: SbiReturnCode = -8;
pub const SBI_ERR_NO_SHMEM: SbiReturnCode = -9;
pub const SBI_ERR_INVALID_STATE: SbiReturnCode = -10;
pub const SBI_ERR_BAD_RANGE: SbiReturnCode = -11;

pub const UNUSED: SseEventState = 0;
pub const REGISTERED: SseEventState = 1;
pub const ENABLED: SseEventState = 2;
pub const RUNNING: SseEventState = 3;

pub struct SseEvent {
    pub event_id: SseEventId,
    pub hart: HartId,
    pub state: SseEventState,
    pub priority: u32,
}

pub struct S {
    pub events: Map<(HartId, SseEventId), SseEvent>,
    pub harts: Set<HartId>,
}

pub open spec fn ResultEqual(a: SbiReturnCode, b: SbiReturnCode) -> bool;

pub open spec fn CallingHart() -> HartId;

pub open spec fn PreExistsRunningEvent(s: S, hart: HartId) -> bool;

pub open spec fn HartSseStateUnchanged(s: S, hart: HartId) -> bool;

pub open spec fn PreHighestPriorityRunningEvent(s: S, hart: HartId) -> SseEvent;

pub open spec fn IsOneShot(e: SseEvent) -> bool;

pub open spec fn InterruptedSupervisorStateResumed(s: S, hart: HartId, e: SseEvent) -> bool;

} // verus!
