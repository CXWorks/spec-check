use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct AXIS_NAME_DESC {
    pub axis_index: u32,
    pub attributes: u32,
}

pub struct S {
    pub sensors: Seq<u32>,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const NOT_FOUND: Int32 = -3;

pub spec const sensor_id: UInt32 = 0;
pub spec const axis_id: UInt32 = 1;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub open spec fn IsValidSensor(s: S, sensor_id: UInt32) -> bool;

pub open spec fn IsValidSensorAxis(s: S, sensor_id: UInt32, axis_id: UInt32) -> bool;

pub open spec fn SensorReportsAxisValues(s: S, sensor_id: UInt32) -> bool;

pub open spec fn NumDescriptors(desc: [AXIS_NAME_DESC]) -> UInt32;

pub open spec fn NumRemainingAxisNameDescriptors(s: S, sensor_id: UInt32, axis_id: UInt32, returned: UInt32) -> UInt32;

} // verus!
