use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt64 = u64;
pub type PeId = u64;
pub type EventId = u64;
pub type EventStateValue = u64;

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const DENIED: Int64 = -3;

pub const UNREGISTERED: EventStateValue = 0;
pub const HANDLER_UNREGISTER_PENDING: EventStateValue = 1;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn CallingPe() -> PeId;

pub open spec fn AnyEventHandlerRunning(pe: PeId) -> bool;

pub open spec fn PrivateEvents(pe: PeId) -> Set<EventId>;

pub open spec fn SharedEvents() -> Set<EventId>;

pub open spec fn EventIsRunning(ev: EventId) -> bool;

pub open spec fn EventState(s: S, ev: EventId) -> EventStateValue;

pub open spec fn EventAuxInfoIsCleared(s: S, ev: EventId) -> bool;

pub open spec fn EventAuxInfoIsWarmBootState(s: S, ev: EventId) -> bool;

} // verus!
