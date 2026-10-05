use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub enum SdeiStatusCode {
    NotSupported,
    InvalidParameters,
    Denied,
    Pending,
    OutOfResource,
}

pub type HandlerState = u32;

pub const HANDLER_UNREGISTERED: HandlerState = 0;
pub const HANDLER_REGISTERED: HandlerState = 1;
pub const HANDLER_UNREGISTER_PENDING: HandlerState = 2;

pub struct EventHandlerInfo {
    pub handler_running: bool,
    pub state: HandlerState,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Result<(), SdeiStatusCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), SdeiStatusCode> = Err(SdeiStatusCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), SdeiStatusCode> = Err(SdeiStatusCode::InvalidParameters);
pub spec const DENIED: Result<(), SdeiStatusCode> = Err(SdeiStatusCode::Denied);
pub spec const PENDING: Result<(), SdeiStatusCode> = Err(SdeiStatusCode::Pending);

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn IsValidEventNumber(s: S, event: Int32) -> bool;
pub open spec fn EventIsRegisteredByClient(s: S, event: Int32) -> bool;
pub open spec fn EventHandler(s: S, event: Int32) -> EventHandlerInfo;
pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;
pub open spec fn IsPrivateEvent(s: S, event: Int32) -> bool;
pub open spec fn ResultEqual(a: Result<(), SdeiStatusCode>, b: Result<(), SdeiStatusCode>) -> bool;

} // verus!
