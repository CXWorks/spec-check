use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct SENSOR_AXIS_DESC {
    pub axis_id: u32,
    pub axis_flags: u32,
    pub resolution: i32,
    pub min_range: i32,
    pub max_range: i32,
}

pub struct S {
    pub num_sensors: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;
pub const NOT_SUPPORTED: Int32 = 2;

pub const result: Int32 = -1;

pub open spec fn IsValidSensor(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorReportsAxisValues(s: S, sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
