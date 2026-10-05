use vstd::prelude::*;

verus! {

pub type UInt64 = u64;
pub type PeId = u64;
pub type EventStateValue = u32;

#[derive(PartialEq, Eq)]
pub enum SdeiStatusCode {
    SUCCESS,
    NOT_SUPPORTED,
    INVALID_PARAMETERS,
    DENIED,
    PENDING,
    OUT_OF_RESOURCE,
}

pub struct Event {
    pub id: u64,
    pub state: EventStateValue,
    pub running: bool,
}

pub struct S {
    pub supported: bool,
    pub calling_pe: PeId,
    pub private_events: Map<PeId, Set<Event>>,
    pub shared_events: Set<Event>,
}

pub spec const SUCCESS: SdeiStatusCode = SdeiStatusCode::SUCCESS;
pub spec const NOT_SUPPORTED: SdeiStatusCode = SdeiStatusCode::NOT_SUPPORTED;
pub spec const DENIED: SdeiStatusCode = SdeiStatusCode::DENIED;

pub spec const UNREGISTERED: EventStateValue = 0;
pub spec const HANDLER_UNREGISTER_PENDING: EventStateValue = 1;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<(), SdeiStatusCode>, code: SdeiStatusCode) -> bool;

pub open spec fn AnyEventHandlerRunning(s: S, pe: PeId) -> bool;

pub open spec fn CallingPe(s: S) -> PeId;

pub open spec fn PrivateEvents(s: S, pe: PeId) -> Set<Event>;

pub open spec fn SharedEvents(s: S) -> Set<Event>;

pub open spec fn EventIsRunning(ev: Event) -> bool;

pub open spec fn EventState(ev: Event) -> EventStateValue;

pub open spec fn EventAuxInfoIsCleared(ev: Event) -> bool;

pub open spec fn EventAuxInfoIsWarmBootState(ev: Event) -> bool;

} // verus!
