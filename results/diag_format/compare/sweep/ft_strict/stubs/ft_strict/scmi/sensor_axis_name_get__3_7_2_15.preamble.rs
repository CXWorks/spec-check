use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: u64,
}

pub struct AXIS_NAME_DESC {
    pub axis_id: UInt32,
    pub name: u64,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const NOT_SUPPORTED: Int32 = -2;

pub open spec fn IsValidSensor(s: S, sensor_id: UInt32) -> bool;

pub open spec fn IsValidSensorAxis(s: S, sensor_id: UInt32, axis_id: UInt32) -> bool;

pub open spec fn SensorReportsValuesAlongAxis(s: S, sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(x: UInt32, hi: u32, lo: u32) -> UInt32;

pub open spec fn DescriptorCount(s: S, desc: [AXIS_NAME_DESC; 1]) -> UInt32;

pub open spec fn RemainingAxisNameDescriptors(s: S, sensor_id: UInt32, axis_id: UInt32, count: UInt32) -> UInt32;

pub open spec fn SensorAxisAtIndex(s: S, sensor_id: UInt32, index: int) -> UInt32;

pub open spec fn IsNullTerminatedUtf8(s: S, name: u64, max_len: int) -> bool;

} // verus!
