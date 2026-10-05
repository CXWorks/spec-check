use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int64 = i64;

pub struct SbiRet {
    pub error: Int64,
    pub value: UInt64,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: Int64 = 0;
pub const SBI_ERR_FAILED: Int64 = -1;

pub const SSE_EVENT_UNUSED: UInt32 = 0;
pub const SSE_EVENT_REGISTERED: UInt32 = 1;
pub const SSE_EVENT_ENABLED: UInt32 = 2;
pub const SSE_EVENT_RUNNING: UInt32 = 3;

pub open spec fn SseEventState(s: S, event_id: UInt32, hart: UInt64) -> UInt32;

pub open spec fn CallingHart(s: S) -> UInt64;

pub open spec fn IsSseLocalEvent(event_id: UInt32) -> bool;

} // verus!
