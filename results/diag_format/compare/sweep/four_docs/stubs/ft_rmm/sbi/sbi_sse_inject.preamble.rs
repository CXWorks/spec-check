use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SbiReturnCode = i64;
pub type HartId = u64;

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
pub const SBI_ERR_TIMEOUT: SbiReturnCode = -12;
pub const SBI_ERR_IO: SbiReturnCode = -13;

pub struct SseEventState {
    pub injected: bool,
    pub enabled: bool,
    pub registered: bool,
    pub running: bool,
    pub priority: u32,
}

pub struct S {
    pub events: Map<(UInt32, HartId), SseEventState>,
    pub global_events: Map<UInt32, SseEventState>,
    pub in_sse_handler: bool,
    pub current_event: UInt32,
}

pub open spec fn EventInjectAllowedByAttribute(s: S, event_id: UInt32) -> bool;
pub open spec fn IsLocalEvent(s: S, event_id: UInt32) -> bool;
pub open spec fn IsGlobalEvent(s: S, event_id: UInt32) -> bool;
pub open spec fn EventInjected(s: S, event_id: UInt32, hart_id: HartId) -> bool;
pub open spec fn InSseHandler(s: S) -> bool;
pub open spec fn EventReady(s: S, event_id: UInt32) -> bool;
pub open spec fn EventPriority(s: S, event_id: UInt32) -> int;
pub open spec fn CurrentEvent() -> UInt32;
pub open spec fn EventHandledImmediately(s: S, event_id: UInt32) -> bool;
pub open spec fn EventRunsAfterCompletion(s: S, event_id: UInt32, current: UInt32) -> bool;
pub open spec fn EventAt(s: S, event_id: UInt32, hart_id: HartId) -> SseEventState;

} // verus!
