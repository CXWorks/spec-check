use vstd::prelude::*;

verus! {

pub type EventId = u32;

pub struct EventHandler {
    pub running: bool,
}

pub struct S {
    pub private_events: Set<EventId>,
    pub event_handlers: Map<EventId, EventHandler>,
    pub sdei_supported: bool,
}

pub const SDEI_SUCCESS: i64 = 0;
pub const SDEI_NOT_SUPPORTED: i64 = -1;
pub const SDEI_DENIED: i64 = -3;

pub uninterp spec fn all_private_events_unregistered(old_s: S, new_s: S) -> bool;

pub uninterp spec fn no_running_handlers(old_s: S, new_s: S) -> bool;

pub uninterp spec fn sdei_not_supported(old_s: S) -> bool;

pub uninterp spec fn exists_running_handler(old_s: S) -> bool;

} // verus!
