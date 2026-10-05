use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub sdei_supported: bool,
    pub handler_running: bool,
}

pub const NOT_SUPPORTED: i64 = -1;
pub const DENIED: i64 = -3;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn SdeiHandlerRunning(s: S) -> bool;

pub open spec fn SdeiEventHandlingCompleted(old_s: S, new_s: S, status_code: UInt32) -> bool;

pub open spec fn SdeiResumesInterruptedContext(old_s: S, new_s: S) -> bool;

} // verus!
