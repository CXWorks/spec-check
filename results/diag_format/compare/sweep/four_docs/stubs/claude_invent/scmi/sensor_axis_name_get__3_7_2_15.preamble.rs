use vstd::prelude::*;

verus! {

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -3;
pub const NOT_SUPPORTED: i32 = -1;

pub struct AxisName {
    pub value: u64,
}

pub struct AxisNameDesc {
    pub axis_id: u32,
    pub name: AxisName,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsValidSensor(s: S, sensor_id: u32) -> bool;

pub open spec fn SensorReportsAxisValues(s: S, sensor_id: u32) -> bool;

pub open spec fn IsValidSensorAxis(s: S, sensor_id: u32, axis_id: u32) -> bool;

pub open spec fn SensorNumAxes(s: S, sensor_id: u32) -> u32;

pub open spec fn SensorAxisExtendedName(s: S, sensor_id: u32, axis_id: u32) -> AxisName;

} // verus!
