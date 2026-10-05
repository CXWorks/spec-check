use vstd::prelude::*;
verus! {

pub type sbiret = i64;

pub type EventId = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SBI_SUCCESS: sbiret = 0;
pub spec const SBI_ERR_NOT_SUPPORTED: sbiret = (-2) as sbiret;
pub spec const SBI_ERR_INVALID_PARAM: sbiret = (-3) as sbiret;
pub spec const SBI_ERR_INVALID_STATE: sbiret = (-10) as sbiret;

pub spec const event_id: EventId = 0;

pub uninterp spec fn EventRegistered(s: S, id: EventId) -> bool;

pub uninterp spec fn EventValid(s: S, id: EventId) -> bool;

pub uninterp spec fn PlatformSupportsEvent(s: S, id: EventId) -> bool;

pub uninterp spec fn EventEnabled(s: S, id: EventId) -> bool;

} // verus!
