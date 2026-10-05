use vstd::prelude::*;
verus! {

pub type Int64 = i64;
pub type EventId = u64;
pub type PeId = u64;
pub type ClientId = u64;
pub type HandlerState = u32;

pub struct S {
    pub dummy: u64,
}

pub spec const event: EventId = 0;

pub spec const NOT_SUPPORTED: Int64 = -1i64;
pub spec const INVALID_PARAMETERS: Int64 = -2i64;
pub spec const DENIED: Int64 = -3i64;
pub spec const SUCCESS: Int64 = 0i64;

pub spec const HANDLER_UNREGISTER_PENDING: HandlerState = 3;

pub uninterp spec fn SdeiIsSupported() -> bool;
pub uninterp spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub uninterp spec fn IsKnownEvent(e: EventId) -> bool;
pub uninterp spec fn EventIsRegisteredByClient(e: EventId) -> bool;
pub uninterp spec fn EventHandlerState(e: EventId) -> HandlerState;
pub uninterp spec fn EventIsPrivate(e: EventId) -> bool;
pub uninterp spec fn EventIsShared(e: EventId) -> bool;
pub uninterp spec fn EventIsEnabledForPe(e: EventId, pe: PeId) -> bool;
pub uninterp spec fn EventIsEnabledForClient(e: EventId, c: ClientId) -> bool;
pub uninterp spec fn CallingPe() -> PeId;
pub uninterp spec fn CallingClient() -> ClientId;

} // verus!
