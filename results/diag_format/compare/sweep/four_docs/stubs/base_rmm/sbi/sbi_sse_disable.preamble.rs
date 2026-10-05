use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type HartId = u64;

pub type SseState = u64;

pub type EventId = u32;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

#[allow(non_upper_case_globals)]
pub const event_id: EventId = 0x10;

pub const UNUSED: SseState = 0;
pub const REGISTERED: SseState = 1;
pub const ENABLED: SseState = 2;
pub const RUNNING: SseState = 3;

pub open spec fn SseEventState(s: S, eid: EventId, hart: HartId) -> SseState;

pub open spec fn ResultIsError(ret: sbiret) -> bool;

pub open spec fn ResultIsSuccess(ret: sbiret) -> bool;

pub open spec fn IsLocalEvent(eid: EventId) -> bool;

pub open spec fn IsGlobalEvent(eid: EventId) -> bool;

pub open spec fn CallingHart() -> HartId;

} // verus!
