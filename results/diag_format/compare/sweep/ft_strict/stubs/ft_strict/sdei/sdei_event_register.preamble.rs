use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt64 = u64;
pub type ResultCode = i32;
pub type ClientId = u64;
pub type PeId = u64;
pub type HandlerState = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: ResultCode = 0;
pub const NOT_SUPPORTED: ResultCode = -1;
pub const INVALID_PARAMETERS: ResultCode = -2;
pub const DENIED: ResultCode = -3;

pub const RM_ANY: u64 = 0;
pub const RM_PE: u64 = 1;

pub const HANDLER_UNREGISTERED: HandlerState = 0;
pub const HANDLER_REGISTERED: HandlerState = 1;
pub const HANDLER_UNREGISTER_PENDING: HandlerState = 2;

pub open spec fn ResultEqual(a: ResultCode, b: ResultCode) -> bool;
pub open spec fn SdeiSupported(s: S) -> bool;
pub open spec fn IsValidEvent(s: S, event: Int32) -> bool;
pub open spec fn DispatcherDetectsInvalidEntryPoint(s: S, entry_point_address: UInt64, flags: UInt64) -> bool;
pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;
pub open spec fn IsValidRoutingMode(s: S, mode: u64) -> bool;
pub open spec fn Bits(value: UInt64, hi: int, lo: int) -> u64;
pub open spec fn IsValidAffinity(s: S, affinity: UInt64) -> bool;
pub open spec fn IsRegisteredByClient(s: S, event: Int32, client: ClientId) -> bool;
pub open spec fn CallingClient(s: S) -> ClientId;
pub open spec fn CallingPe(s: S) -> PeId;
pub open spec fn EventHandlerState(s: S, event: Int32) -> HandlerState;
pub open spec fn IsRegisteredForClient(s: S, event: Int32, client: ClientId) -> bool;
pub open spec fn IsRegisteredForPe(s: S, event: Int32, pe: PeId) -> bool;
pub open spec fn EventEnabled(s: S, event: Int32) -> bool;
pub open spec fn EventEntryPoint(s: S, event: Int32) -> UInt64;
pub open spec fn ResolveEntryPoint(s: S, entry_point_address: UInt64, mode: u64) -> UInt64;
pub open spec fn EventArgument(s: S, event: Int32) -> UInt64;
pub open spec fn EventRoutingMode(s: S, event: Int32) -> u64;
pub open spec fn EventAffinity(s: S, event: Int32) -> UInt64;

} // verus!
