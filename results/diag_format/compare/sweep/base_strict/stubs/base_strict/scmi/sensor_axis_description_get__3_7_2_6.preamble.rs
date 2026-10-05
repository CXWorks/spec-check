use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub sensors: Seq<u32>,
    pub current_sensor_id: u32,
    pub current_axis_desc_index: u32,
}

pub struct SensorAxisDescriptor {
    pub axis_attributes_low: u32,
    pub axis_attributes_high: u32,
    pub name: Seq<u8>,
    pub axis_resolution: u32,
    pub axis_min_range_low: u32,
    pub axis_min_range_high: u32,
    pub axis_max_range_low: u32,
    pub axis_max_range_high: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const NOT_SUPPORTED: Int32 = -2;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn sensor_id(s: S) -> UInt32;

pub open spec fn axis_desc_index(s: S) -> UInt32;

pub open spec fn IsValidSensor(s: S, id: UInt32) -> bool;

pub open spec fn SensorReportsAxisValues(s: S, id: UInt32) -> bool;

pub open spec fn Bits(x: UInt32, hi: UInt32, lo: UInt32) -> UInt32;

pub open spec fn DescCount(desc: [SensorAxisDescriptor]) -> UInt32;

pub open spec fn DescAt(desc: [SensorAxisDescriptor], i: UInt32) -> SensorAxisDescriptor;

pub open spec fn NumSensorAxes(s: S, id: UInt32) -> UInt32;

pub open spec fn SensorAxisDescriptor(s: S, id: UInt32, idx: int) -> SensorAxisDescriptor;

pub open spec fn AxisNameLongerThan16Bytes(s: S, id: UInt32, idx: int) -> bool;

pub open spec fn IsNullTerminatedUtf8(name: Seq<u8>, max_len: int) -> bool;

pub open spec fn ExtendedAttributeFieldsAllocated(d: SensorAxisDescriptor) -> bool;

pub open spec fn SensorReportsAxisResolution(s: S, id: UInt32, idx: int) -> bool;

pub open spec fn SensorReportsAxisMinRange(s: S, id: UInt32, idx: int) -> bool;

pub open spec fn SensorReportsAxisMaxRange(s: S, id: UInt32, idx: int) -> bool;

} // verus!
