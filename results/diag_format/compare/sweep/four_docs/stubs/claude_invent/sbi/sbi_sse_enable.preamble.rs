use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type HartId = u64;
pub type SbiError = i64;
pub type SseState = u32;

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_NOT_SUPPORTED: SbiError = -2;
pub const SBI_ERR_INVALID_PARAM: SbiError = -3;
pub const SBI_ERR_INVALID_STATE: SbiError = -10;

pub const UNUSED: SseState = 0;
pub const REGISTERED: SseState = 1;
pub const ENABLED: SseState = 2;
pub const RUNNING: SseState = 3;

pub open spec fn SseEventIdValid(s: S, event_id: UInt32) -> bool;
pub open spec fn SseEventSupported(s: S, event_id: UInt32) -> bool;
pub open spec fn SseEventState(s: S, event_id: UInt32, h: HartId) -> SseState;
pub open spec fn CallingHart(s: S) -> HartId;
pub open spec fn SseEventIsGlobal(s: S, event_id: UInt32) -> bool;
pub open spec fn IsHart(s: S, h: HartId) -> bool;

} // verus!
