use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub const RSI_INCOMPLETE: RsiCommandReturnCode = 3;
pub const RSI_ERROR_UNKNOWN: RsiCommandReturnCode = 4;

pub type SdeiEventNumber = u64;

pub struct S {
    pub sdei_event_register_count: u64,
    pub sdei_event_unregister_event: SdeiEventNumber,
    pub sdei_event_handler_running: bool,
}

impl S {
    pub open spec fn sdei_event_registered(self, event: SdeiEventNumber) -> bool;
}

} // verus!
