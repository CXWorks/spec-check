use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INVALID_PARAMETERS: RsiCommandReturnCode = 1;
pub const RSI_ERROR_DENIED: RsiCommandReturnCode = 2;

pub enum SdeiEventState {
    SDEI_EVENT_UNREGISTERED,
    SDEI_EVENT_REGISTERED,
    SDEI_EVENT_ENABLED,
    SDEI_EVENT_RUNNING,
}

pub struct SdeiEvent {
    pub state: SdeiEventState,
    pub interrupt: int,
}

pub struct S {
    pub sdei_events: Seq<SdeiEvent>,
}

pub open spec fn SdeiEventAt(s: S, event: int) -> SdeiEvent;

} // verus!
