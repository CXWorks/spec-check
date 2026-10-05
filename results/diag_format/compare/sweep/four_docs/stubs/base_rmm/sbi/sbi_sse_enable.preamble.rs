use vstd::prelude::*;

verus! {

pub type SbiReturnCode = i64;
pub type EventId = u32;
pub type HartId = u64;
pub type SseEventState = u8;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: SbiReturnCode = 0;
pub const SBI_ERR_NOT_SUPPORTED: SbiReturnCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiReturnCode = -3;
pub const SBI_ERR_INVALID_STATE: SbiReturnCode = -10;

pub const UNUSED: SseEventState = 0;
pub const REGISTERED: SseEventState = 1;
pub const ENABLED: SseEventState = 2;
pub const RUNNING: SseEventState = 3;

pub const event_id: EventId = 0;

pub open spec fn IsReservedEventId(s: S, id: EventId) -> bool;
pub open spec fn IsValidEventId(s: S, id: EventId) -> bool;
pub open spec fn PlatformSupportsEvent(s: S, id: EventId) -> bool;
pub open spec fn ResultEqual(r: SbiReturnCode, code: SbiReturnCode) -> bool;
pub open spec fn EventState(s: S, id: EventId) -> SseEventState;
pub open spec fn IsLocalEvent(s: S, id: EventId) -> bool;
pub open spec fn IsGlobalEvent(s: S, id: EventId) -> bool;
pub open spec fn CallingHart() -> HartId;

} // verus!
