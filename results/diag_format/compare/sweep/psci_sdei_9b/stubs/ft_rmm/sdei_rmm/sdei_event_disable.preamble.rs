use vstd::prelude::*;
verus! {

pub type Int32 = i32;

pub type SdeiCommandReturnCode = i32;

pub type HandlerState = u32;

pub type PeId = u64;

pub type ClientId = u64;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: SdeiCommandReturnCode = 0;
pub spec const NOT_SUPPORTED: SdeiCommandReturnCode = (-1int) as i32;
pub spec const INVALID_PARAMETERS: SdeiCommandReturnCode = (-2int) as i32;
pub spec const DENIED: SdeiCommandReturnCode = (-3int) as i32;

pub spec const SDEI_SUCCESS: Result<(), SdeiCommandReturnCode> = Ok(());

pub spec const HANDLER_UNREGISTER_PENDING: HandlerState = 3;

pub uninterp spec fn SdeiIsSupported(s: S) -> bool;

pub uninterp spec fn ResultEqual(result: Result<(), SdeiCommandReturnCode>, code: SdeiCommandReturnCode) -> bool;

pub uninterp spec fn IsKnownEvent(s: S, event: Int32) -> bool;

pub uninterp spec fn EventIsRegisteredByClient(s: S, event: Int32) -> bool;

pub uninterp spec fn EventHandlerState(s: S, event: Int32) -> HandlerState;

pub uninterp spec fn EventIsPrivate(s: S, event: Int32) -> bool;

pub uninterp spec fn EventIsShared(s: S, event: Int32) -> bool;

pub uninterp spec fn EventIsEnabledForPe(s: S, event: Int32, pe: PeId) -> bool;

pub uninterp spec fn EventIsEnabledForClient(s: S, event: Int32, client: ClientId) -> bool;

pub uninterp spec fn CallingPe() -> PeId;

pub uninterp spec fn CallingClient() -> ClientId;

} // verus!
