use vstd::prelude::*;
verus! {

pub struct S {
    pub sdei_supported: bool,
    pub state_id: int,
}

pub const NOT_SUPPORTED: i64 = -1i64;
pub const INVALID_PARAMETERS: i64 = -2i64;
pub const DENIED: i64 = -3i64;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn SdeiEventIsValid(s: S, event: i32) -> bool;

pub open spec fn SdeiEventIsShared(s: S, event: i32) -> bool;

pub open spec fn SdeiEventIsHandlerRegistered(s: S, event: i32) -> bool;

pub open spec fn SdeiEventRoutingMode(s: S, event: i32) -> int;

pub open spec fn SdeiEventRoutingAffinity(s: S, event: i32) -> int;

pub open spec fn SdeiEventCanBeSoftwareSignaled(s: S, event: i32) -> bool;

pub open spec fn SdeiEventIsCriticalPriority(s: S, event: i32) -> bool;

} // verus!
