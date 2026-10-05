use vstd::prelude::*;
verus! {

pub type EventId = u64;

pub const SDEI_SUCCESS: i64 = 0;
pub const SDEI_ERROR_INVALID_PARAMETERS: i64 = -2;
pub const SDEI_ERROR_DENIED: i64 = -3;

pub const SDEI_EVENT_STATE_UNREGISTERED: u32 = 0;
pub const SDEI_EVENT_STATE_HANDLER_REGISTERED: u32 = 1;

pub struct S {
    pub event: EventId,
}

impl S {
    pub open spec fn sdei_event_state(self, event: EventId) -> u32;

    pub open spec fn sdei_event_handler(self, event: EventId) -> ();

    pub open spec fn sdei_event_argument(self, event: EventId) -> u64;
}

} // verus!
