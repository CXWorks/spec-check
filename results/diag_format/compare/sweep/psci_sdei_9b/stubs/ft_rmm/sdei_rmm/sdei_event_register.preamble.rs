use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type Address = u64;

pub struct UInt64 {
    pub value: u64,
    pub routing_mode: u64,
    pub relative_mode: u64,
}

pub enum SdeiCommandReturnCode {
    NotSupported,
    InvalidParameters,
    Denied,
    Pending,
    OutOfResource,
}

pub struct SdeiEventHandler {
    pub entry_point_address: Address,
    pub relative_mode: u64,
    pub ep_argument: UInt64,
}

pub struct S {
    pub dummy: u64,
}

pub type HandlerStateT = u32;

pub spec const SUCCESS: Result<(), SdeiCommandReturnCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), SdeiCommandReturnCode> = Err(SdeiCommandReturnCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), SdeiCommandReturnCode> = Err(SdeiCommandReturnCode::InvalidParameters);
pub spec const DENIED: Result<(), SdeiCommandReturnCode> = Err(SdeiCommandReturnCode::Denied);

pub spec const RM_ANY: u64 = 0;
pub spec const RM_PE: u64 = 1;

pub spec const HANDLER_UNREGISTERED: HandlerStateT = 0;
pub spec const HANDLER_REGISTERED: HandlerStateT = 1;
pub spec const HANDLER_UNREGISTER_PENDING: HandlerStateT = 2;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(a: Result<(), SdeiCommandReturnCode>, b: Result<(), SdeiCommandReturnCode>) -> bool;
pub open spec fn IsValidEvent(s: S, event: Int32) -> bool;
pub open spec fn DispatcherCanDetermineInvalidAddress(s: S, addr: Address) -> bool;
pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;
pub open spec fn IsPrivateEvent(s: S, event: Int32) -> bool;
pub open spec fn IsValidRoutingMode(s: S, mode: u64) -> bool;
pub open spec fn IsValidMpidr(s: S, affinity: UInt64) -> bool;
pub open spec fn IsRegisteredByClient(s: S, event: Int32) -> bool;
pub open spec fn EventHandlerState(s: S, event: Int32) -> HandlerStateT;
pub open spec fn EventHandler(s: S, event: Int32) -> SdeiEventHandler;
pub open spec fn IsEnabled(s: S, event: Int32) -> bool;
pub open spec fn HandlerRegisteredGloballyForClient(s: S, event: Int32) -> bool;
pub open spec fn HandlerRegisteredForCallingPe(s: S, event: Int32) -> bool;
pub open spec fn RoutingMode(s: S, event: Int32) -> u64;
pub open spec fn RoutingAffinity(s: S, event: Int32) -> UInt64;

} // verus!
