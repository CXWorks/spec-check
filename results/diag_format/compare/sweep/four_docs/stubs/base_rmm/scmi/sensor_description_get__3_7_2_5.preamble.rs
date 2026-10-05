use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub struct SensorDescriptor {
    pub sensor_id: UInt32,
    pub sensor_attributes_low: UInt32,
    pub sensor_attributes_high: UInt32,
    pub sensor_name: Seq<UInt8>,
    pub sensor_power: UInt32,
    pub sensor_resolution: UInt32,
    pub sensor_min_range_low: UInt32,
    pub sensor_min_range_high: UInt32,
    pub sensor_max_range_low: UInt32,
    pub sensor_max_range_high: UInt32,
}

pub struct S {
    pub sensors: Seq<SensorDescriptor>,
}

#[allow(non_upper_case_globals)]
pub spec const desc_index: int = 0;

} // verus!
