use vstd::prelude::*;
verus! {

pub type SdeiCommandReturnCode = i32;

pub type Interrupt = u32;

pub struct SdeiEvent {
    pub id: u32,
    pub handler_running: bool,
}

pub struct S {
    pub sdei_supported: bool,
}

pub spec const SUCCESS: SdeiCommandReturnCode = 0;
pub spec const NOT_SUPPORTED: SdeiCommandReturnCode = (-1) as i32;
pub spec const DENIED: SdeiCommandReturnCode = (-2) as i32;

pub spec const SDEI_SUCCESS: Result<(), SdeiCommandReturnCode> = Ok(());

pub uninterp spec fn IsSdeiSupported(s: S) -> bool;

pub uninterp spec fn ResultEqual(result: Result<(), SdeiCommandReturnCode>, code: SdeiCommandReturnCode) -> bool;

pub uninterp spec fn IsSharedEvent(e: SdeiEvent) -> bool;

pub uninterp spec fn IsPrivateEvent(e: SdeiEvent) -> bool;

pub uninterp spec fn IsInterruptBoundEvent(e: SdeiEvent) -> bool;

pub uninterp spec fn IsEventRegistered(e: SdeiEvent) -> bool;

pub uninterp spec fn IsInterruptBoundToEvent(i: Interrupt) -> bool;

pub uninterp spec fn SharedAuxiliaryInfoCleared() -> bool;

pub uninterp spec fn PrivateEventStateUnchanged(e: SdeiEvent) -> bool;

} // verus!
