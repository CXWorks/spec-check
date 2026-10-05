use vstd::prelude::*;

verus! {

pub type SbiErrorCode = i64;
pub type EventId = u32;
pub type HartId = u64;
pub type SseState = u8;

pub struct S {
    pub dummy: u64,
}

pub struct SseAttr {
    pub ENTRY_PC: u64,
    pub ENTRY_ARG: u64,
}

pub struct SseEventInfo {
    pub state: SseState,
    pub attr: SseAttr,
}

pub spec const SBI_SUCCESS: SbiErrorCode = 0;
pub spec const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = (-2int) as i64;
pub spec const SBI_ERR_INVALID_PARAM: SbiErrorCode = (-3int) as i64;
pub spec const SBI_ERR_INVALID_STATE: SbiErrorCode = (-10int) as i64;

pub spec const UNUSED: SseState = 0;
pub spec const REGISTERED: SseState = 1;

pub spec const event_id: EventId = 0;
pub spec const handler_entry_pc: u64 = 0;
pub spec const handler_entry_arg: u64 = 1;

pub open spec fn IsValidEventId(id: EventId) -> bool;
pub open spec fn IsAligned(addr: u64, align: u64) -> bool;
pub open spec fn PlatformSupportsEvent(id: EventId) -> bool;
pub open spec fn IsLocalEvent(id: EventId) -> bool;
pub open spec fn IsGlobalEvent(id: EventId) -> bool;
pub open spec fn CallingHart() -> HartId;
pub open spec fn SseEvent(id: EventId) -> SseEventInfo;
pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

} // verus!
