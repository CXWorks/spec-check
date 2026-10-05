use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type SensorId = u32;

pub struct SENSOR_AXIS_DESC {
    pub axis: u32,
    pub flags: u32,
}

pub struct S {
    pub current_sensor: SensorId,
    pub sensor_count: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;
pub const NOT_SUPPORTED: Int32 = 2;

pub open spec fn sensor_id(s: S) -> SensorId;

pub open spec fn IsValidSensor(id: SensorId) -> bool;

pub open spec fn SensorReportsAxisValues(id: SensorId) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn ValidSensorAxisDescriptor(d: SENSOR_AXIS_DESC, id: SensorId) -> bool;

pub open spec fn SensorAxisDescriptorOrder(d: SENSOR_AXIS_DESC, id: SensorId) -> bool;

} // verus!
