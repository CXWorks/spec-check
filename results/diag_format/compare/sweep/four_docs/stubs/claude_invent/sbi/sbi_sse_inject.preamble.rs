use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub struct S {
    pub dummy: u64,
}

pub open spec fn SseEventInjectionAllowed(s: S, event_id: UInt32) -> bool;

pub open spec fn SseEventInjected(old_s: S, new_s: S, event_id: UInt32, hart_id: UInt64) -> bool;

} // verus!
