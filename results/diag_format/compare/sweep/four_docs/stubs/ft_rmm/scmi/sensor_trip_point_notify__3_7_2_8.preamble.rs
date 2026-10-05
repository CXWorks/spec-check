use vstd::prelude::*;
verus! {

pub type UInt32 = Seq<u32>;
pub type Int32 = i32;

pub struct Sensor {
    pub trip_point_notify_enabled: u32,
}

pub struct S {
    pub sensors: Seq<Sensor>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_SUPPORTED: Int32 = -3;

#[allow(non_upper_case_globals)]
pub const result: Int32 = -100;

pub open spec fn IsExistingSensor(s: S, sensor_id: UInt32) -> bool;

pub open spec fn IsValidSensorEventControl(s: S, sensor_event_control: UInt32) -> bool;

pub open spec fn SensorSupportsTripPointNotify(s: S, sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn SensorAt(s: S, sensor_id: UInt32) -> Sensor;

} // verus!
