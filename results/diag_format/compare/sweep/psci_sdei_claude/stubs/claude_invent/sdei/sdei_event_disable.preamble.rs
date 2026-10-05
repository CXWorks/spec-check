use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: i64 = -1;
pub const INVALID_PARAMETERS: i64 = -2;
pub const DENIED: i64 = -3;
pub const SUCCESS: i64 = 0;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn SdeiEventIsValid(s: S, event: i32) -> bool;
pub open spec fn SdeiEventIsRegisteredByClient(s: S, event: i32) -> bool;
pub open spec fn SdeiEventIsHandlerUnregisterPending(s: S, event: i32) -> bool;
pub open spec fn SdeiEventIsEnabled(s: S, event: i32) -> bool;
pub open spec fn SdeiEventIsPending(s: S, event: i32) -> bool;
pub open spec fn SdeiEventIsRunning(s: S, event: i32) -> bool;

} // verus!
