use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub sensor_ids: Set<UInt32>,
    pub trip_point_notifications_enabled: Map<UInt32, bool>,
    pub trip_point_event_support: Map<UInt32, bool>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_FOUND: Int32 = -4;

#[allow(non_upper_case_globals)]
pub const result: Int32 = 7;

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;

pub open spec fn IsValidSensorEventControl(s: S, sensor_event_control: UInt32) -> bool;

pub open spec fn SupportsTripPointEventNotifications(s: S, sensor_id: UInt32) -> bool;

pub open spec fn TripPointNotificationsEnabled(s: S, sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> int;

} // verus!
