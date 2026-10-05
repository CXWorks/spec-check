use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt64 = u64;
pub type Int64 = i64;
pub type HandlerStateT = u32;

pub struct EventHandlerInfo {
    pub entry_point_address: UInt64,
    pub relative_mode: UInt64,
    pub ep_argument: UInt64,
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub const RM_ANY: UInt64 = 0;
pub const RM_PE: UInt64 = 1;

pub const HANDLER_UNREGISTERED: HandlerStateT = 0;
pub const HANDLER_REGISTERED: HandlerStateT = 1;
pub const HANDLER_UNREGISTER_PENDING: HandlerStateT = 2;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub open spec fn IsValidEvent(s: S, event: Int32) -> bool;
pub open spec fn DispatcherCanDetermineInvalidAddress(s: S, addr: UInt64) -> bool;
pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;
pub open spec fn IsPrivateEvent(s: S, event: Int32) -> bool;
pub open spec fn IsValidRoutingMode(mode: UInt64) -> bool;
pub open spec fn IsValidMpidr(s: S, affinity: UInt64) -> bool;
pub open spec fn IsRegisteredByClient(s: S, event: Int32) -> bool;
pub open spec fn EventHandlerState(s: S, event: Int32) -> HandlerStateT;
pub open spec fn EventHandler(s: S, event: Int32) -> EventHandlerInfo;
pub open spec fn IsEnabled(s: S, event: Int32) -> bool;
pub open spec fn HandlerRegisteredGloballyForClient(s: S, event: Int32) -> bool;
pub open spec fn HandlerRegisteredForCallingPe(s: S, event: Int32) -> bool;
pub open spec fn RoutingMode(s: S, event: Int32) -> UInt64;
pub open spec fn RoutingAffinity(s: S, event: Int32) -> UInt64;

} // verus!
