use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct SENSOR_DESC {
    pub sensor_id: UInt32,
    pub sensor_name: Seq<u8>,
    pub sensor_attributes_low: UInt32,
    pub sensor_attributes_high: UInt32,
    pub sensor_power: UInt32,
    pub sensor_resolution: UInt32,
    pub sensor_min_range_low: UInt32,
    pub sensor_min_range_high: UInt32,
    pub sensor_max_range_low: UInt32,
    pub sensor_max_range_high: UInt32,
}

pub struct S {
    pub state: int,
}

pub const SUCCESS: Int32 = 0;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn Bits(value: UInt32, high: int, low: int) -> UInt32;

pub open spec fn NumRemainingSensorDescriptors(desc_index: UInt32, requested: UInt32) -> UInt32;

pub open spec fn SensorDescriptorAt(index: int) -> SENSOR_DESC;

pub open spec fn SensorDescriptorIsSameOnRepeatedCalls(sensor_id: UInt32) -> bool;

pub open spec fn IsNullTerminatedUtf8(s: Seq<u8>, max_len: int) -> bool;

pub open spec fn ExtendedAttributesAllocated(desc: SENSOR_DESC) -> bool;

pub open spec fn SensorReportsPower(sensor_id: UInt32) -> bool;

pub open spec fn SensorReportsResolution(sensor_id: UInt32) -> bool;

pub open spec fn SensorReportsMinRange(sensor_id: UInt32) -> bool;

pub open spec fn SensorReportsMaxRange(sensor_id: UInt32) -> bool;

} // verus!
