use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SbiErrorCode = i64;
pub type Hart = u64;
pub type SseStateT = u32;

pub struct S {
    pub calling_hart: Hart,
    pub dummy: int,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_INVALID_STATE: SbiErrorCode = -10;

pub const SSE_STATE_UNUSED: SseStateT = 0;
pub const SSE_STATE_REGISTERED: SseStateT = 1;

pub open spec fn SseEventIdIsValid(event_id: UInt32) -> bool;
pub open spec fn SseEventIsSupported(s: S, event_id: UInt32) -> bool;
pub open spec fn SseEventState(s: S, h: Hart, event_id: UInt32) -> SseStateT;
pub open spec fn CallingHart(s: S) -> Hart;
pub open spec fn SseEventIsGlobal(event_id: UInt32) -> bool;
pub open spec fn SseEventEntryPc(s: S, h: Hart, event_id: UInt32) -> UInt64;
pub open spec fn SseEventEntryArg(s: S, h: Hart, event_id: UInt32) -> UInt64;
pub open spec fn IsValidHart(s: S, h: Hart) -> bool;

} // verus!
