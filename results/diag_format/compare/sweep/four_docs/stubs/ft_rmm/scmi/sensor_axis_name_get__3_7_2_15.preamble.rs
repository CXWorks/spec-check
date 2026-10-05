use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct AXIS_NAME_DESC {
    pub axis_id: UInt32,
    pub name: Seq<u8>,
}

pub struct S {
    pub sensors: Set<UInt32>,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = -1;
pub spec const NOT_SUPPORTED: Int32 = -2;
pub spec const result: Int32 = 1;

pub open spec fn IsValidSensor(s: S, sensor_id: UInt32) -> bool;

pub open spec fn IsValidSensorAxis(s: S, sensor_id: UInt32, axis_id: UInt32) -> bool;

pub open spec fn SensorReportsAxisValues(s: S, sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn NumDescriptors(s: S, desc: [AXIS_NAME_DESC; 1]) -> UInt32;

pub open spec fn NumRemainingAxisNameDescriptors(s: S, sensor_id: UInt32, axis_id: UInt32, num_returned: UInt32) -> UInt32;

} // verus!
