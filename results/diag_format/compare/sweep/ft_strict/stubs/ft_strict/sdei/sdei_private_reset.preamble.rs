use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type Result<A, B> = B;

pub struct S {
    pub sdei_supported: bool,
}

pub struct SdeiEvent {
    pub id: u64,
}

#[allow(non_camel_case_types)]
pub enum SdeiCommandReturnCode {
    SUCCESS,
    NOT_SUPPORTED,
    INVALID_PARAMETERS,
    DENIED,
    PENDING,
    OUT_OF_RESOURCE,
}

pub spec const SUCCESS: SdeiCommandReturnCode = SdeiCommandReturnCode::SUCCESS;
pub spec const NOT_SUPPORTED: SdeiCommandReturnCode = SdeiCommandReturnCode::NOT_SUPPORTED;
pub spec const DENIED: SdeiCommandReturnCode = SdeiCommandReturnCode::DENIED;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(result: SdeiCommandReturnCode, code: SdeiCommandReturnCode) -> bool;
pub open spec fn IsPrivateEvent(e: SdeiEvent) -> bool;
pub open spec fn IsSharedEvent(e: SdeiEvent) -> bool;
pub open spec fn EventOwnerPe(e: SdeiEvent) -> UInt64;
pub open spec fn CallingPe() -> UInt64;
pub open spec fn HandlerRunning(e: SdeiEvent) -> bool;
pub open spec fn EventIsUnregistered(e: SdeiEvent) -> bool;
pub open spec fn EventIsRegistered(e: SdeiEvent) -> bool;
pub open spec fn HandlerUnregisterPending(e: SdeiEvent) -> bool;
pub open spec fn PrivateEventAuxInfoReset(pe: UInt64) -> bool;
pub open spec fn EventStateUnchanged(e: SdeiEvent) -> bool;

} // verus!
