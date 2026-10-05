use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i64 = 0;
pub const NOT_SUPPORTED: i64 = -1;
pub const INVALID_PARAMETERS: i64 = -2;
pub const DENIED: i64 = -3;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn SdeiEventIsKnown(s: S, event: i32) -> bool;
pub open spec fn SdeiEventIsRegisteredByCaller(s: S, event: i32) -> bool;
pub open spec fn SdeiEventHandlerIsUnregisterPending(s: S, event: i32) -> bool;
pub open spec fn SdeiEventIsEnabledForCaller(s: S, event: i32) -> bool;
pub open spec fn SdeiEventIsPrivate(s: S, event: i32) -> bool;
pub open spec fn SdeiEventEnabledOnlyForCallingPe(old_s: S, new_s: S, event: i32) -> bool;
pub open spec fn SdeiEventEnabledGloballyForCallingClient(old_s: S, new_s: S, event: i32) -> bool;
pub open spec fn SdeiOtherEventsUnchanged(old_s: S, new_s: S, event: i32) -> bool;

} // verus!
