use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type SbiReturnCode = i64;

pub type HartId = u32;

pub type SseEventState = u32;

pub struct S {
    pub calling_hart: HartId,
    pub reserved_event_ids: Set<UInt32>,
    pub valid_event_ids: Set<UInt32>,
    pub supported_event_ids: Set<UInt32>,
    pub local_event_ids: Set<UInt32>,
    pub global_event_ids: Set<UInt32>,
    pub event_states: Map<UInt32, SseEventState>,
}

pub const SBI_SUCCESS: SbiReturnCode = 0;
pub const SBI_ERR_NOT_SUPPORTED: SbiReturnCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiReturnCode = -3;
pub const SBI_ERR_INVALID_STATE: SbiReturnCode = -10;

pub const UNUSED: SseEventState = 0;
pub const REGISTERED: SseEventState = 1;
pub const ENABLED: SseEventState = 2;
pub const RUNNING: SseEventState = 3;

pub open spec fn IsReservedEventId(s: S, event_id: UInt32) -> bool;

pub open spec fn IsValidEventId(s: S, event_id: UInt32) -> bool;

pub open spec fn PlatformSupportsEvent(s: S, event_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: SbiReturnCode, expected: SbiReturnCode) -> bool;

pub open spec fn EventState(s: S, event_id: UInt32) -> SseEventState;

pub open spec fn IsLocalEvent(s: S, event_id: UInt32) -> bool;

pub open spec fn IsGlobalEvent(s: S, event_id: UInt32) -> bool;

pub open spec fn CallingHart(s: S) -> HartId;

} // verus!
