use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type InterruptId = u32;

pub type EventNumber = i64;

pub struct S {
    pub interrupts: Seq<InterruptId>,
    pub bound_events: Map<InterruptId, EventNumber>,
}

#[derive(PartialEq, Eq)]
pub enum InterruptStateKind {
    INACTIVE,
    ACTIVE,
    PENDING,
}

pub use InterruptStateKind::INACTIVE;

#[derive(PartialEq, Eq)]
pub enum PriorityKind {
    NORMAL,
    CRITICAL,
}

pub use PriorityKind::NORMAL;

pub spec const interrupt: InterruptId = 0;

pub spec const NOT_SUPPORTED: Int64 = -1;
pub spec const INVALID_PARAMETERS: Int64 = -2;
pub spec const DENIED: Int64 = -3;
pub spec const OUT_OF_RESOURCE: Int64 = -10;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn IsValidInterrupt(s: S, intr: InterruptId) -> bool;

pub open spec fn IsInterruptAllowedForBinding(s: S, intr: InterruptId) -> bool;

pub open spec fn InterruptState(s: S, intr: InterruptId) -> InterruptStateKind;

pub open spec fn IsInterruptBound(s: S, intr: InterruptId) -> bool;

pub open spec fn BindSlotAvailable(s: S) -> bool;

pub open spec fn BoundEventNumber(s: S, intr: InterruptId) -> Int64;

pub open spec fn EventPriority(ev: Int64) -> PriorityKind;

pub open spec fn IsPpi(s: S, intr: InterruptId) -> bool;

pub open spec fn IsSpi(s: S, intr: InterruptId) -> bool;

pub open spec fn IsPrivateEvent(ev: Int64) -> bool;

pub open spec fn IsSharedEvent(ev: Int64) -> bool;

pub open spec fn PreviousBoundEventNumber(s: S, intr: InterruptId) -> Int64;

pub open spec fn IsVendorEventNumber(ev: Int64) -> bool;

pub open spec fn IsInterruptPriorityElevated(s: S, intr: InterruptId) -> bool;

} // verus!
