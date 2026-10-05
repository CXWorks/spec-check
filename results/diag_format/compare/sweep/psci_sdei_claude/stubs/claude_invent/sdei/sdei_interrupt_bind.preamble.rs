use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: i64 = -1;
pub const INVALID_PARAMETERS: i64 = -2;
pub const DENIED: i64 = -3;
pub const OUT_OF_RESOURCE: i64 = -10;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn SdeiInterruptIsValid(s: S, interrupt: UInt32) -> bool;

pub open spec fn SdeiInterruptIsSgi(s: S, interrupt: UInt32) -> bool;

pub open spec fn SdeiInterruptIsPpi(s: S, interrupt: UInt32) -> bool;

pub open spec fn SdeiInterruptIsSpi(s: S, interrupt: UInt32) -> bool;

pub open spec fn SdeiInterruptOwnedByClient(s: S, interrupt: UInt32) -> bool;

pub open spec fn SdeiInterruptIsBound(s: S, interrupt: UInt32) -> bool;

pub open spec fn SdeiInterruptIsInactive(s: S, interrupt: UInt32) -> bool;

pub open spec fn SdeiHasFreeBindSlot(s: S) -> bool;

pub open spec fn SdeiBoundEventNumber(s: S, interrupt: UInt32) -> UInt32;

pub open spec fn SdeiIsVendorEventNumber(s: S, event: UInt32) -> bool;

pub open spec fn SdeiEventIsNormalPriority(s: S, event: UInt32) -> bool;

pub open spec fn SdeiEventIsPrivate(s: S, event: UInt32) -> bool;

pub open spec fn SdeiEventIsShared(s: S, event: UInt32) -> bool;

pub open spec fn SdeiBindSlotsUsed(s: S) -> nat;

} // verus!
