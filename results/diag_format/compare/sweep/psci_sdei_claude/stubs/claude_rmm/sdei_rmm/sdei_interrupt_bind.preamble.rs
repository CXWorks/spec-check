use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int64 = i64;
pub type UInt64 = u64;

pub struct S {
    pub sdei_supported: bool,
    pub bind_slots_free: nat,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;
pub const OUT_OF_RESOURCE: Int64 = -10;

pub const INACTIVE: UInt32 = 0;
pub const ACTIVE: UInt32 = 1;

pub const NORMAL: UInt32 = 0;
pub const CRITICAL: UInt32 = 1;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub open spec fn IsValidInterrupt(s: S, interrupt: UInt32) -> bool;
pub open spec fn IsInterruptAllowedForBinding(s: S, interrupt: UInt32) -> bool;
pub open spec fn InterruptState(s: S, interrupt: UInt32) -> UInt32;
pub open spec fn IsInterruptBound(s: S, interrupt: UInt32) -> bool;
pub open spec fn BindSlotAvailable(s: S) -> bool;
pub open spec fn BoundEventNumber(s: S, interrupt: UInt32) -> UInt64;
pub open spec fn EventPriority(s: S, event: UInt64) -> UInt32;
pub open spec fn IsPpi(s: S, interrupt: UInt32) -> bool;
pub open spec fn IsSpi(s: S, interrupt: UInt32) -> bool;
pub open spec fn IsPrivateEvent(s: S, event: UInt64) -> bool;
pub open spec fn IsSharedEvent(s: S, event: UInt64) -> bool;
pub open spec fn PreviousBoundEventNumber(s: S, interrupt: UInt32) -> UInt64;
pub open spec fn IsVendorEventNumber(s: S, event: UInt64) -> bool;
pub open spec fn IsInterruptPriorityElevated(s: S, interrupt: UInt32) -> bool;

} // verus!
