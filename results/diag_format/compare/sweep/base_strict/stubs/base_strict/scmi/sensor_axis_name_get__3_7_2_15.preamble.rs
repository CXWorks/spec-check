use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct AXIS_NAME_DESC {
    pub axis_id: UInt32,
    pub name: Seq<u8>,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const NOT_FOUND: Int32 = -4;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn sensor_id(s: S) -> UInt32;

pub open spec fn axis_id(s: S) -> UInt32;

pub open spec fn IsValidSensor(s: S, sensor_id: UInt32) -> bool;

pub open spec fn IsValidSensorAxis(s: S, sensor_id: UInt32, axis_id: UInt32) -> bool;

pub open spec fn SensorReportsValuesAlongAxis(s: S, sensor_id: UInt32) -> bool;

pub open spec fn Bits64(value: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn DescriptorCount(desc: [AXIS_NAME_DESC; 0]) -> UInt32;

pub open spec fn RemainingAxisNameDescriptors(s: S, sensor_id: UInt32, axis_id: UInt32, count: UInt32) -> UInt32;

pub open spec fn SensorAxisAtIndex(s: S, sensor_id: UInt32, index: int) -> UInt32;

pub open spec fn IsNullTerminatedUtf8(name: Seq<u8>, max_len: int) -> bool;

} // verus!
