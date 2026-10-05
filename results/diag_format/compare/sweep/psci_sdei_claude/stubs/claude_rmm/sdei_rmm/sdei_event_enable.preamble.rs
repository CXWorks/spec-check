use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type Int64 = i64;
pub type PeId = u64;
pub type ClientId = u64;
pub type HandlerState = u32;

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub const HANDLER_UNREGISTERED: HandlerState = 0;
pub const HANDLER_REGISTERED: HandlerState = 1;
pub const HANDLER_UNREGISTER_PENDING: HandlerState = 2;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;
pub open spec fn IsKnownEvent(s: S, event: Int32) -> bool;
pub open spec fn IsEventRegisteredByClient(s: S, event: Int32) -> bool;
pub open spec fn EventHandlerState(s: S, event: Int32) -> HandlerState;
pub open spec fn IsPrivateEvent(s: S, event: Int32) -> bool;
pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;
pub open spec fn IsEventEnabledForPe(s: S, event: Int32, pe: PeId) -> bool;
pub open spec fn IsEventEnabledForClient(s: S, event: Int32, client: ClientId) -> bool;
pub open spec fn CallingPe(s: S) -> PeId;
pub open spec fn CallingClient(s: S) -> ClientId;

} // verus!
