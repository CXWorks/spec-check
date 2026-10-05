use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn SensorExists(sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub open spec fn IsValidSensorEventControl(sensor_event_control: UInt32) -> bool;

pub open spec fn SupportsTripPointEventNotifications(sensor_id: UInt32) -> bool;

pub open spec fn Bits(value: UInt32, high: int, low: int) -> int;

pub open spec fn TripPointNotificationsEnabled(sensor_id: UInt32) -> bool;

} // verus!
