use vstd::prelude::*;
verus! {

pub type Int64 = i64;
pub type UInt64 = u64;
pub type EventId = u64;
pub type PeId = u64;
pub type ClientId = u64;
pub type HandlerState = u64;

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

#[allow(non_upper_case_globals)]
pub const event: EventId = 7;

pub open spec fn SdeiIsSupported() -> bool;
pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub open spec fn IsKnownEvent(e: EventId) -> bool;
pub open spec fn IsEventRegisteredByClient(e: EventId) -> bool;
pub open spec fn EventHandlerState(e: EventId) -> HandlerState;
pub open spec fn IsPrivateEvent(e: EventId) -> bool;
pub open spec fn IsSharedEvent(e: EventId) -> bool;
pub open spec fn IsEventEnabledForPe(e: EventId, pe: PeId) -> bool;
pub open spec fn IsEventEnabledForClient(e: EventId, c: ClientId) -> bool;
pub open spec fn CallingPe() -> PeId;
pub open spec fn CallingClient() -> ClientId;

} // verus!
