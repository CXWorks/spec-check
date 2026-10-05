use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt64 = u64;
pub type UInt32 = u32;

pub struct S {
    pub state_id: u64,
}

pub struct SdeiFlags {
    pub routing_mode: UInt64,
    pub relative_mode: UInt64,
}

pub struct HandlerInfo {
    pub entry_point_address: UInt64,
    pub relative_mode: UInt64,
    pub ep_argument: UInt64,
}

pub spec const SUCCESS: Int64 = 0;
pub spec const NOT_SUPPORTED: Int64 = (-1) as i64;
pub spec const INVALID_PARAMETERS: Int64 = (-2) as i64;
pub spec const DENIED: Int64 = (-3) as i64;

pub spec const HANDLER_UNREGISTER_PENDING: UInt32 = 7;

pub spec const RM_ANY: UInt64 = 0;
pub spec const RM_PE: UInt64 = 1;

pub spec const event: UInt32 = 42;
pub spec const entry_point_address: UInt64 = 4096;
pub spec const ep_argument: UInt64 = 8192;
pub spec const affinity: UInt64 = 12288;
pub spec const flags: SdeiFlags = SdeiFlags { routing_mode: 0, relative_mode: 1 };

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;
pub open spec fn IsValidEvent(s: S, ev: UInt32) -> bool;
pub open spec fn DispatcherCanDetermineInvalidAddress(s: S, addr: UInt64) -> bool;
pub open spec fn IsSharedEvent(s: S, ev: UInt32) -> bool;
pub open spec fn IsPrivateEvent(s: S, ev: UInt32) -> bool;
pub open spec fn IsValidRoutingMode(s: S, mode: UInt64) -> bool;
pub open spec fn IsValidMpidr(s: S, mpidr: UInt64) -> bool;
pub open spec fn IsRegisteredByClient(s: S, ev: UInt32) -> bool;
pub open spec fn EventHandlerState(s: S, ev: UInt32) -> UInt32;
pub open spec fn EventHandler(s: S, ev: UInt32) -> HandlerInfo;
pub open spec fn IsEnabled(s: S, ev: UInt32) -> bool;
pub open spec fn HandlerRegisteredGloballyForClient(s: S, ev: UInt32) -> bool;
pub open spec fn HandlerRegisteredForCallingPe(s: S, ev: UInt32) -> bool;
pub open spec fn RoutingMode(s: S, ev: UInt32) -> UInt64;
pub open spec fn RoutingAffinity(s: S, ev: UInt32) -> UInt64;

} // verus!
