use vstd::prelude::*;

verus! {

pub type uint32_t = u32;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: int,
}

pub type SseState = u8;

pub const ENABLED: SseState = 2;
pub const REGISTERED: SseState = 1;

pub open spec fn SseEventState(s: S, event_id: uint32_t, hart: int) -> SseState;

pub open spec fn ResultIsError(ret: sbiret) -> bool;

pub open spec fn ResultIsSuccess(ret: sbiret) -> bool;

pub open spec fn IsLocalEvent(s: S, event_id: uint32_t) -> bool;

pub open spec fn IsGlobalEvent(s: S, event_id: uint32_t) -> bool;

pub open spec fn CallingHart() -> int;

} // verus!
