use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Address = u64;
pub type UInt = u64;
pub type SbiErrorCode = i64;
pub type HartId = u64;
pub type SseEventState = u8;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const SBI_ERR_INVALID_STATE: SbiErrorCode = -10;

pub const UNUSED: SseEventState = 0;
pub const REGISTERED: SseEventState = 1;
pub const ENABLED: SseEventState = 2;
pub const RUNNING: SseEventState = 3;

pub struct SseEventAttr {
    pub ENTRY_PC: Address,
    pub ENTRY_ARG: UInt,
}

pub struct SseEventInfo {
    pub state: SseEventState,
    pub attr: SseEventAttr,
}

pub struct S {
    pub calling_hart: HartId,
}

pub open spec fn IsValidEventId(s: S, event_id: UInt32) -> bool;

pub open spec fn IsAligned(s: S, addr: Address, align: int) -> bool;

pub open spec fn PlatformSupportsEvent(s: S, event_id: UInt32) -> bool;

pub open spec fn IsLocalEvent(s: S, event_id: UInt32) -> bool;

pub open spec fn IsGlobalEvent(s: S, event_id: UInt32) -> bool;

pub open spec fn CallingHart(s: S) -> HartId;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

// NOTE: the function calls SseEvent with both 2 arguments (global event) and
// 3 arguments (local event, per hart). Rust/Verus does not support overloading
// or variadic functions, so no single declaration can type-check both call
// forms. This is the 3-argument (per-hart) form; the 2-argument calls must be
// rewritten (e.g. to SseEventGlobal) for the function to type-check.
pub open spec fn SseEvent(s: S, event_id: UInt32, hart: HartId) -> SseEventInfo;

pub open spec fn SseEventGlobal(s: S, event_id: UInt32) -> SseEventInfo;

} // verus!
