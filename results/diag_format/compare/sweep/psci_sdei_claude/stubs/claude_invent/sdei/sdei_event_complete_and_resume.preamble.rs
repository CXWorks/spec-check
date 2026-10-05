use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Int64 = i64;

pub struct S {
    pub supported: bool,
    pub handler_running: bool,
    pub resume_addr: u64,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn SdeiHandlerRunning(s: S) -> bool;
pub open spec fn SdeiResumeAddrIsValid(s: S, addr: UInt64) -> bool;
pub open spec fn SdeiEventHandlingCompleted(old_s: S, new_s: S) -> bool;
pub open spec fn SdeiResumedAtAddr(s: S, addr: UInt64) -> bool;

} // verus!
