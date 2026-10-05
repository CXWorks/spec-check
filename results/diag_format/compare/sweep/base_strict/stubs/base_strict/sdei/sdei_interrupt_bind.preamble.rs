use vstd::prelude::*;
verus! {

pub type Int64 = i64;

pub type Interrupt = u32;

pub type PE = u64;

pub type EventNumber = i64;

pub type InterruptStateT = u32;

pub type PriorityT = u32;

pub struct S {
    pub interrupt: Interrupt,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;
pub const OUT_OF_RESOURCE: Int64 = -10;

pub const INACTIVE: InterruptStateT = 0;

pub const NORMAL_PRIORITY: PriorityT = 0;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn IsValidInterrupt(s: S, intr: Interrupt) -> bool;

pub open spec fn IsPpi(s: S, intr: Interrupt) -> bool;

pub open spec fn IsSpi(s: S, intr: Interrupt) -> bool;

pub open spec fn IsOwnedByClient(s: S, intr: Interrupt) -> bool;

pub open spec fn InterruptState(s: S, intr: Interrupt) -> InterruptStateT;

pub open spec fn IsBound(s: S, intr: Interrupt) -> bool;

pub open spec fn NumFreeBindSlots(s: S) -> int;

pub open spec fn Bits(x: Int64, hi: int, lo: int) -> EventNumber;

pub open spec fn IsVendorEventNumber(ev: EventNumber) -> bool;

pub open spec fn EventIsBoundToInterrupt(ev: EventNumber, intr: Interrupt) -> bool;

pub open spec fn EventPriority(ev: EventNumber) -> PriorityT;

pub open spec fn IsPrivateEvent(ev: EventNumber) -> bool;

pub open spec fn IsSharedEvent(ev: EventNumber) -> bool;

pub open spec fn EventNumberIsValidOnPe(ev: EventNumber, pe: PE) -> bool;

pub open spec fn Old<T>(x: T) -> T;

pub open spec fn BoundEventNumber(s: S, intr: Interrupt) -> EventNumber;

pub open spec fn InterruptPriorityIsElevated(s: S, intr: Interrupt) -> bool;

pub open spec fn InterruptIsManagedByDispatcher(s: S, intr: Interrupt) -> bool;

} // verus!
