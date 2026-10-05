use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u64;

pub type SdeiEventId = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub enum SdeiEventHandlerState {
    SdeiEventHandlerStateHandlerUnregistered,
    SdeiEventHandlerStateHandlerRegistered,
    SdeiEventHandlerStateHandlerUnregisterPending,
}

pub use SdeiEventHandlerState::*;

pub struct S {
    pub sdei_event_id: SdeiEventId,
}

impl S {
    pub open spec fn sdei_event_register(&self, id: SdeiEventId) -> bool;

    pub open spec fn sdei_event_handler_state(&self, id: SdeiEventId) -> SdeiEventHandlerState;

    pub open spec fn sdei_event_enabled(&self, id: SdeiEventId) -> bool;
}

} // verus!
