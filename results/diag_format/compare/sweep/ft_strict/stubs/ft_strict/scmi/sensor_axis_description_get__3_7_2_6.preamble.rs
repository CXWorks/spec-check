use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub struct SENSOR_AXIS_DESC {
    pub axis_attributes_low: UInt32,
    pub axis_resolution: UInt32,
    pub axis_min_range_low: UInt32,
    pub axis_min_range_high: UInt32,
    pub axis_max_range_low: UInt32,
    pub axis_max_range_high: UInt32,
    pub name: Seq<u8>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const NOT_SUPPORTED: Int32 = -2;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidSensor(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorReportsAxisValues(s: S, sensor_id: UInt32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> int;

pub open spec fn DescCount(s: S, desc: [SENSOR_AXIS_DESC; 1]) -> int;

pub open spec fn NumSensorAxes(s: S, sensor_id: UInt32) -> int;

pub open spec fn DescAt(s: S, desc: [SENSOR_AXIS_DESC; 1], i: UInt32) -> SENSOR_AXIS_DESC;

pub open spec fn SensorAxisDescriptor(s: S, sensor_id: UInt32, idx: int) -> SENSOR_AXIS_DESC;

pub open spec fn AxisNameLongerThan16Bytes(s: S, sensor_id: UInt32, idx: int) -> bool;

pub open spec fn IsNullTerminatedUtf8(s: S, name: Seq<u8>, len: int) -> bool;

pub open spec fn ExtendedAttributeFieldsAllocated(s: S, d: SENSOR_AXIS_DESC) -> bool;

pub open spec fn SensorReportsAxisResolution(s: S, sensor_id: UInt32, idx: int) -> bool;

pub open spec fn SensorReportsAxisMinRange(s: S, sensor_id: UInt32, idx: int) -> bool;

pub open spec fn SensorReportsAxisMaxRange(s: S, sensor_id: UInt32, idx: int) -> bool;

} // verus!
