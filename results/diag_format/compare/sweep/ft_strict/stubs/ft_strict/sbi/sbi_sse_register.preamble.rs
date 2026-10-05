use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Address = u64;
pub type SbiReturnCode = i64;
pub type Hart = u64;
pub type SseEventState = u32;
pub type SseAttributeId = u32;

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: SbiReturnCode = 0;
pub const SBI_ERR_NOT_SUPPORTED: SbiReturnCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiReturnCode = -3;
pub const SBI_ERR_INVALID_STATE: SbiReturnCode = -10;

pub const UNUSED: SseEventState = 0;
pub const REGISTERED: SseEventState = 1;

pub const ENTRY_PC: SseAttributeId = 6;
pub const ENTRY_ARG: SseAttributeId = 7;

pub open spec fn IsValidEventId(s: S, event_id: UInt32) -> bool;
pub open spec fn IsReservedEventId(s: S, event_id: UInt32) -> bool;
pub open spec fn PlatformSupportsEvent(s: S, event_id: UInt32) -> bool;
pub open spec fn IsLocalEvent(s: S, event_id: UInt32) -> bool;
pub open spec fn IsGlobalEvent(s: S, event_id: UInt32) -> bool;
pub open spec fn EventState(s: S, event_id: UInt32) -> SseEventState;
pub open spec fn EventStateForHart(s: S, event_id: UInt32, h: Hart) -> SseEventState;
pub open spec fn CallingHart() -> Hart;
pub open spec fn EventAttribute(s: S, event_id: UInt32, attr: SseAttributeId) -> u64;
pub open spec fn ResultEqual(a: SbiReturnCode, b: SbiReturnCode) -> bool;

} // verus!
