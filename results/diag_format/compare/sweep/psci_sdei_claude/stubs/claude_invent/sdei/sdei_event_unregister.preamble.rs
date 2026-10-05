use vstd::prelude::*;

verus! {

pub struct S {
    pub sdei_supported: bool,
    pub registered: Map<i32, bool>,
    pub unregister_pending: Map<i32, bool>,
    pub handler_running: Map<i32, bool>,
}

pub const SUCCESS: i64 = 0;
pub const NOT_SUPPORTED: i64 = -1;
pub const INVALID_PARAMETERS: i64 = -2;
pub const DENIED: i64 = -3;
pub const PENDING: i64 = -5;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn SdeiEventIsValid(s: S, event: i32) -> bool;

pub open spec fn SdeiEventIsRegistered(s: S, event: i32) -> bool;

pub open spec fn SdeiEventIsUnregisterPending(s: S, event: i32) -> bool;

pub open spec fn SdeiEventHandlerRunning(s: S, event: i32) -> bool;

} // verus!
