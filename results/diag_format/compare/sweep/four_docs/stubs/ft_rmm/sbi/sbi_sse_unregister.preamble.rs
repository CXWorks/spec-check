use vstd::prelude::*;
verus! {

pub type uint32_t = u32;

pub type SbiCommandReturnCode = u64;

pub type SseState = u64;

pub struct S {
    pub dummy: int,
}

pub const RSI_SUCCESS: SbiCommandReturnCode = 0;
pub const RSI_ERROR_STATE: SbiCommandReturnCode = 1;

pub const UNUSED: SseState = 0;
pub const REGISTERED: SseState = 1;

pub open spec fn SseEventState(s: S, event_id: uint32_t) -> SseState;

pub open spec fn IsLocalEvent(s: S, event_id: uint32_t) -> bool;

pub open spec fn IsGlobalEvent(s: S, event_id: uint32_t) -> bool;

pub open spec fn IsHandlerRegistered(s: S, event_id: uint32_t) -> bool;

} // verus!
