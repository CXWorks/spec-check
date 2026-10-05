use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type Hart = u64;

pub type EventStateT = u32;

pub const ENABLED: EventStateT = 2;
pub const REGISTERED: EventStateT = 1;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn EventState(s: S, event_id: UInt32, h: Hart) -> EventStateT;

pub open spec fn CallingHart(s: S) -> Hart;

pub open spec fn ResultIsError(r: sbiret) -> bool;

pub open spec fn ResultIsSuccess(r: sbiret) -> bool;

pub open spec fn IsLocalEvent(s: S, event_id: UInt32) -> bool;

pub open spec fn IsGlobalEvent(s: S, event_id: UInt32) -> bool;

} // verus!
