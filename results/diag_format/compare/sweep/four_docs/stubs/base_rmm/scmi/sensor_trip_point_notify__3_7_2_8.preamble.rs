use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_FOUND: Int32 = -4;

pub struct SensorState {
    pub trip_point_notify_enabled: int,
}

pub struct S {
    pub sensors: Map<UInt32, SensorState>,
}

pub open spec fn IsExistingSensor(sensor_id: UInt32) -> bool;

pub open spec fn IsValidSensorEventControl(sensor_event_control: UInt32) -> bool;

pub open spec fn SensorSupportsTripPointNotify(sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub open spec fn SensorAt(s: S, sensor_id: UInt32) -> SensorState;

} // verus!
