use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub num_clocks: UInt32,
    pub clock_get_permissions_implemented: bool,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = -4;
pub spec const result: Int32 = -100;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn IsValidClockId(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockSupportsRateChangeNotifications(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockSupportsRateChangeRequestedNotifications(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockNameLength(s: S, clock_id: UInt32) -> UInt32;

pub open spec fn ClockAdvertisesParentIds(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockSupportsExtendedConfig(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockHasRestrictions(s: S, clock_id: UInt32) -> bool;

pub open spec fn IsClockGetPermissionsImplemented(s: S) -> bool;

pub open spec fn ClockIsEnabled(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockNameString(s: S, clock_id: UInt32) -> [UInt8; 16];

pub open spec fn ClockWorstCaseEnableDelayUs(s: S, clock_id: UInt32) -> UInt32;

} // verus!
