use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type EventNumber = u32;
pub type ClientId = u32;
pub type PeId = u64;
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

#[allow(non_upper_case_globals)]
pub const event: EventNumber = 0;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn IsKnownEventNumber(e: EventNumber) -> bool;

pub open spec fn CallingClient() -> ClientId;

pub open spec fn CallingPe() -> PeId;

pub open spec fn IsEventRegisteredByClient(e: EventNumber, c: ClientId) -> bool;

pub open spec fn EventHandlerState(e: EventNumber, c: ClientId) -> HandlerState;

pub open spec fn IsPrivateEvent(e: EventNumber) -> bool;

pub open spec fn IsSharedEvent(e: EventNumber) -> bool;

pub open spec fn IsEventEnabledForPe(e: EventNumber, pe: PeId) -> bool;

pub open spec fn IsEventEnabledForClient(e: EventNumber, c: ClientId) -> bool;

pub open spec fn IsEventPending(e: EventNumber) -> bool;

pub open spec fn EventDeliveredWhenEnabled(e: EventNumber, c: ClientId) -> bool;

} // verus!
