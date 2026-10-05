use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int64 = i64;

pub struct S {
    pub dummy: int,
}

pub struct PE {
    pub id: int,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;
pub const OUT_OF_RESOURCE: Int64 = -10;

pub const INACTIVE: u32 = 0;

pub const NORMAL_PRIORITY: u32 = 0;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub open spec fn IsValidInterrupt(s: S, interrupt: UInt32) -> bool;
pub open spec fn IsPpi(s: S, interrupt: UInt32) -> bool;
pub open spec fn IsSpi(s: S, interrupt: UInt32) -> bool;
pub open spec fn IsOwnedByClient(s: S, interrupt: UInt32) -> bool;
pub open spec fn InterruptState(s: S, interrupt: UInt32) -> u32;
pub open spec fn IsBound(s: S, interrupt: UInt32) -> bool;
pub open spec fn NumFreeBindSlots(s: S) -> int;
pub open spec fn Bits(x: Int64, hi: int, lo: int) -> int;
pub open spec fn IsVendorEventNumber(ev: int) -> bool;
pub open spec fn EventIsBoundToInterrupt(s: S, ev: int, interrupt: UInt32) -> bool;
pub open spec fn EventPriority(s: S, ev: int) -> u32;
pub open spec fn IsPrivateEvent(s: S, ev: int) -> bool;
pub open spec fn IsSharedEvent(s: S, ev: int) -> bool;
pub open spec fn EventNumberIsValidOnPe(s: S, ev: int, pe: PE) -> bool;
pub open spec fn BoundEventNumber(s: S, interrupt: UInt32) -> int;
pub open spec fn InterruptPriorityIsElevated(s: S, interrupt: UInt32) -> bool;
pub open spec fn InterruptIsManagedByDispatcher(s: S, interrupt: UInt32) -> bool;

} // verus!
