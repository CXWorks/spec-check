use vstd::prelude::*;

verus! {

pub type SdeiCommandReturnCode = i64;

pub spec const SUCCESS: SdeiCommandReturnCode = 0i64;
pub spec const NOT_SUPPORTED: SdeiCommandReturnCode = -1i64;
pub spec const DENIED: SdeiCommandReturnCode = -3i64;

pub spec const SDEI_SUCCESS: Result<(), SdeiCommandReturnCode> = Ok(());

pub struct S {
    pub dummy: int,
}

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<(), SdeiCommandReturnCode>, code: SdeiCommandReturnCode) -> bool;

pub open spec fn AnySharedEventHandlerRunning(s: S) -> bool;

pub open spec fn AnyInterruptEventBindingRegistered(s: S) -> bool;

pub open spec fn AnySharedEventRegistered(s: S) -> bool;

pub open spec fn AnyInterruptBoundToEvent(s: S) -> bool;

pub open spec fn SharedEventAuxInfoCleared(s: S) -> bool;

pub open spec fn InterruptBindingAuxInfoCleared(s: S) -> bool;

pub open spec fn PrivateEventState(s: S) -> int;

} // verus!
