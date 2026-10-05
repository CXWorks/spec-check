use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type SdeiCommandReturnCode = i64;

pub type HandlerState = u32;

pub type PeId = u64;

pub type ClientId = u64;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: SdeiCommandReturnCode = 0;
pub spec const NOT_SUPPORTED: SdeiCommandReturnCode = (-1int) as i64;
pub spec const INVALID_PARAMETERS: SdeiCommandReturnCode = (-2int) as i64;
pub spec const DENIED: SdeiCommandReturnCode = (-3int) as i64;

pub spec const SDEI_SUCCESS: Result<(), SdeiCommandReturnCode> = Ok(());

pub spec const HANDLER_UNREGISTER_PENDING: HandlerState = 3;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<(), SdeiCommandReturnCode>, code: SdeiCommandReturnCode) -> bool;

pub open spec fn IsKnownEvent(s: S, event: Int32) -> bool;

pub open spec fn IsEventRegisteredByClient(s: S, event: Int32) -> bool;

pub open spec fn EventHandlerState(s: S, event: Int32) -> HandlerState;

pub open spec fn IsPrivateEvent(s: S, event: Int32) -> bool;

pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;

pub open spec fn IsEventEnabledForPe(s: S, event: Int32, pe: PeId) -> bool;

pub open spec fn IsEventEnabledForClient(s: S, event: Int32, client: ClientId) -> bool;

pub open spec fn CallingPe() -> PeId;

pub open spec fn CallingClient() -> ClientId;

} // verus!
