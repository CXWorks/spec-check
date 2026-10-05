use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;
pub type ClockId = u32;

pub struct S {
    pub clock_id: ClockId,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn clock_id(s: S) -> ClockId;
pub open spec fn IsValidClockId(id: ClockId) -> bool;
pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;
pub open spec fn ClockSupportsRateChangeNotifications(id: ClockId) -> bool;
pub open spec fn ClockSupportsRateChangeRequestedNotifications(id: ClockId) -> bool;
pub open spec fn ClockNameLength(id: ClockId) -> int;
pub open spec fn ClockAdvertisesParentIds(id: ClockId) -> bool;
pub open spec fn ClockSupportsExtendedConfig(id: ClockId) -> bool;
pub open spec fn ClockHasRestrictions(id: ClockId) -> bool;
pub open spec fn IsClockGetPermissionsImplemented() -> bool;
pub open spec fn ClockIsEnabled(id: ClockId) -> bool;
pub open spec fn ClockNameString(id: ClockId) -> Seq<UInt8>;
pub open spec fn ClockWorstCaseEnableDelayUs(id: ClockId) -> UInt32;

} // verus!
