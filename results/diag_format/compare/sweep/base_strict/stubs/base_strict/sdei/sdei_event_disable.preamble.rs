use vstd::prelude::*;
verus! {

pub type Int64 = i64;
pub type EventNumber = u32;
pub type ClientId = u64;
pub type PeId = u64;
pub type HandlerState = u32;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: Int64 = 0;
pub spec const NOT_SUPPORTED: Int64 = (-1int) as i64;
pub spec const INVALID_PARAMETERS: Int64 = (-2int) as i64;
pub spec const DENIED: Int64 = (-3int) as i64;

pub spec const HANDLER_UNREGISTER_PENDING: HandlerState = 1;

pub spec const event: EventNumber = 0;

pub open spec fn SdeiIsSupported() -> bool;
pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;
pub open spec fn IsValidEventNumber(ev: EventNumber) -> bool;
pub open spec fn CallingClient() -> ClientId;
pub open spec fn CallingPe() -> PeId;
pub open spec fn IsEventRegisteredByClient(ev: EventNumber, client: ClientId) -> bool;
pub open spec fn EventHandlerState(ev: EventNumber) -> HandlerState;
pub open spec fn IsPrivateEvent(ev: EventNumber) -> bool;
pub open spec fn IsSharedEvent(ev: EventNumber) -> bool;
pub open spec fn IsEventEnabledForPe(ev: EventNumber, pe: PeId) -> bool;
pub open spec fn IsEventEnabledForClient(ev: EventNumber, client: ClientId) -> bool;
pub open spec fn RunningEventHandlerUnaffected(ev: EventNumber) -> bool;
pub open spec fn TriggeredEventsStayPendingUntilEnabled(ev: EventNumber) -> bool;

} // verus!
