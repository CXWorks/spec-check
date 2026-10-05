use vstd::prelude::*;
verus! {

pub struct S {
    pub sdei_supported: bool,
}

pub const NOT_SUPPORTED: i64 = -1;
pub const INVALID_PARAMETERS: i64 = -2;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn SdeiEventIsValid(s: S, event: i32) -> bool;

pub open spec fn SdeiEventHandlerRunning(s: S, event: i32) -> bool;

pub open spec fn SdeiEventHandlerEnabled(s: S, event: i32) -> bool;

pub open spec fn SdeiEventHandlerRegistered(s: S, event: i32) -> bool;

} // verus!
