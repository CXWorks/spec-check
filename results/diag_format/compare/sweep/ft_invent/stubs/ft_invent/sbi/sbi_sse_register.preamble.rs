use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;
pub const SBI_ERR_INVALID_STATE: SbiCommandReturnCode = -10;

pub type SseEventState = u8;

pub const UNUSED: SseEventState = 0;
pub const REGISTERED: SseEventState = 1;
pub const ENABLED: SseEventState = 2;
pub const RUNNING: SseEventState = 3;

pub struct SseEvent {
    pub state: SseEventState,
    pub handler_entry_pc: UInt64,
    pub handler_entry_arg: UInt64,
}

pub struct S {
    pub events: Map<UInt32, SseEvent>,
}

pub open spec fn EventAt(s: S, event_id: UInt32) -> SseEvent;

} // verus!
