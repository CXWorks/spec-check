use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type SbiErrorCode = i64;
pub type Hart = u64;
pub type EventState = u32;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_INVALID_STATE: SbiErrorCode = -10;

pub const UNUSED: EventState = 0;
pub const REGISTERED: EventState = 1;
pub const ENABLED: EventState = 2;
pub const RUNNING: EventState = 3;

pub struct SseEvent {
    pub state: EventState,
}

pub struct S {
    pub calling_hart: Hart,
    pub events: Map<UInt32, SseEvent>,
}

pub open spec fn IsValidEventId(s: S, event_id: UInt32) -> bool;
pub open spec fn IsReservedEventId(s: S, event_id: UInt32) -> bool;
pub open spec fn PlatformSupportsEvent(s: S, event_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;
pub open spec fn EventAt(s: S, event_id: UInt32) -> SseEvent;
pub open spec fn IsLocalEvent(s: S, event_id: UInt32) -> bool;
pub open spec fn IsGlobalEvent(s: S, event_id: UInt32) -> bool;
pub open spec fn EventEnabledOnHart(s: S, event_id: UInt32, h: Hart) -> bool;
pub open spec fn CallingHart(s: S) -> Hart;

} // verus!
