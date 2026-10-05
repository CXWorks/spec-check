use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt16 = u32;

pub const SUCCESS: Int32 = 0;

pub struct SENSOR_DESC {
    pub sensor_id: u32,
    pub sensor_name: u64,
    pub sensor_attributes_low: u32,
    pub sensor_attributes_high: u32,
    pub sensor_power: u32,
    pub sensor_resolution: u32,
    pub sensor_min_range_low: u32,
    pub sensor_min_range_high: u32,
    pub sensor_max_range_low: u32,
    pub sensor_max_range_high: u32,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn Bits(x: u32, hi: int, lo: int) -> u32;

pub open spec fn NumRemainingSensorDescriptors(s: S, desc_index: UInt32, num: int) -> u32;

pub open spec fn SensorDescriptorAt(s: S, index: int) -> SENSOR_DESC;

pub open spec fn SensorDescriptorIsSameOnRepeatedCalls(s: S, sensor_id: u32) -> bool;

pub open spec fn IsNullTerminatedUtf8(s: S, name: u64, max_len: int) -> bool;

pub open spec fn ExtendedAttributesAllocated(s: S, d: SENSOR_DESC) -> bool;

pub open spec fn SensorReportsPower(s: S, sensor_id: u32) -> bool;

pub open spec fn SensorReportsResolution(s: S, sensor_id: u32) -> bool;

pub open spec fn SensorReportsMinRange(s: S, sensor_id: u32) -> bool;

pub open spec fn SensorReportsMaxRange(s: S, sensor_id: u32) -> bool;

} // verus!
