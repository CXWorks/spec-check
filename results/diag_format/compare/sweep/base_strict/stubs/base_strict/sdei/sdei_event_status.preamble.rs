use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type EventNumber = u64;

pub struct S {
    pub registered: Map<EventNumber, bool>,
    pub enabled: Map<EventNumber, bool>,
    pub running: Map<EventNumber, bool>,
}

pub const NOT_SUPPORTED: Int64 = -1;

pub const INVALID_PARAMETERS: Int64 = -2;

#[allow(non_upper_case_globals)]
pub const event: EventNumber = 0;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn IsKnownEventNumber(ev: EventNumber) -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn Bits(value: Int64, hi: int, lo: int) -> int;

pub open spec fn EventHandlerIsRunning(ev: EventNumber) -> bool;

pub open spec fn EventHandlerIsEnabled(ev: EventNumber) -> bool;

pub open spec fn EventHandlerIsRegistered(ev: EventNumber) -> bool;

pub open spec fn EventStatusMapsToHandlerState(result: Int64, ev: EventNumber) -> bool;

} // verus!
