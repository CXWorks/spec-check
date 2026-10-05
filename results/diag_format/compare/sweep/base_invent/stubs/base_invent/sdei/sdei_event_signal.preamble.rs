use vstd::prelude::*;
verus! {

pub type int64 = i64;

pub const SDEI_EVENT_SIGNAL_SUCCESS: int64 = 0;
pub const SDEI_EVENT_SIGNAL_NOT_SUPPORTED: int64 = -1;
pub const SDEI_EVENT_SIGNAL_INVALID_PARAMETERS: int64 = -2;

pub struct S {
    pub sdei_event_0_signaled: bool,
    pub sdei_supported: bool,
}

} // verus!
