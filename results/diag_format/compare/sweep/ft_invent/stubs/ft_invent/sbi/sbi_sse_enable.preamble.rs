use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type SbiCommandReturnCode = i64;

pub const SBI_ERR_INVALID_STATE: SbiCommandReturnCode = -6;

pub type SseEventState = u32;

pub const REGISTERED: SseEventState = 1;

pub const ENABLED: SseEventState = 2;

pub struct SseEvent {
    pub state: SseEventState,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn EventAt(s: S, event_id: UInt32) -> SseEvent;

} // verus!
