use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type SdeiCommandReturnCode = i64;
pub type HandlerState = u32;
pub type ClientId = u64;
pub type PeId = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: SdeiCommandReturnCode = 0;
pub spec const NOT_SUPPORTED: SdeiCommandReturnCode = (-1) as i64;
pub spec const INVALID_PARAMETERS: SdeiCommandReturnCode = (-2) as i64;
pub spec const DENIED: SdeiCommandReturnCode = (-3) as i64;

pub spec const SDEI_SUCCESS: Result<(), SdeiCommandReturnCode> = Ok(());

pub spec const HANDLER_UNREGISTER_PENDING: HandlerState = 1;

pub uninterp spec fn SdeiIsSupported(s: S) -> bool;
pub uninterp spec fn ResultEqual(result: Result<(), SdeiCommandReturnCode>, code: SdeiCommandReturnCode) -> bool;
pub uninterp spec fn IsValidEventNumber(s: S, event: Int32) -> bool;
pub uninterp spec fn IsEventRegisteredByClient(s: S, event: Int32, client: ClientId) -> bool;
pub uninterp spec fn CallingClient(s: S) -> ClientId;
pub uninterp spec fn CallingPe(s: S) -> PeId;
pub uninterp spec fn EventHandlerState(s: S, event: Int32) -> HandlerState;
pub uninterp spec fn IsPrivateEvent(s: S, event: Int32) -> bool;
pub uninterp spec fn IsSharedEvent(s: S, event: Int32) -> bool;
pub uninterp spec fn IsEventEnabledForPe(s: S, event: Int32, pe: PeId) -> bool;
pub uninterp spec fn IsEventEnabledForClient(s: S, event: Int32, client: ClientId) -> bool;
pub uninterp spec fn RunningEventHandlerUnaffected(s: S, event: Int32) -> bool;
pub uninterp spec fn TriggeredEventsStayPendingUntilEnabled(s: S, event: Int32) -> bool;

} // verus!
