use vstd::prelude::*;

verus! {

pub type SbiErrorCode = i64;
pub type EventId = u32;
pub type Hart = u64;
pub type EventState = u8;

pub struct S {
    pub dummy: u64,
}

pub struct SseEvent {
    pub state: EventState,
}

pub spec const SBI_SUCCESS: SbiErrorCode = 0;
pub spec const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = (-2) as i64;
pub spec const SBI_ERR_INVALID_PARAM: SbiErrorCode = (-3) as i64;
pub spec const SBI_ERR_INVALID_STATE: SbiErrorCode = (-6) as i64;

pub spec const REGISTERED: EventState = 1;
pub spec const ENABLED: EventState = 2;

pub spec const event_id: EventId = 0;

pub uninterp spec fn IsValidEventId(s: S, id: EventId) -> bool;
pub uninterp spec fn IsReservedEventId(s: S, id: EventId) -> bool;
pub uninterp spec fn PlatformSupportsEvent(s: S, id: EventId) -> bool;
pub uninterp spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;
pub uninterp spec fn EventAt(s: S, id: EventId) -> SseEvent;
pub uninterp spec fn IsLocalEvent(id: EventId) -> bool;
pub uninterp spec fn IsGlobalEvent(id: EventId) -> bool;
pub uninterp spec fn EventEnabledOnHart(s: S, id: EventId, h: Hart) -> bool;
pub uninterp spec fn CallingHart() -> Hart;

} // verus!
