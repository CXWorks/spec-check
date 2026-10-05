use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_FOUND: int32 = 1;
pub spec const NOT_SUPPORTED: int32 = 2;

pub spec const axis_desc_index: uint32 = 0;

pub open spec fn uint32(x: int) -> uint32;

pub struct SensorAxisDescriptor {
    pub axis_id: uint32,
    pub axis_attributes_low: uint32,
    pub axis_attributes_high: uint32,
    pub name: Seq<char>,
    pub axis_resolution: uint32,
    pub axis_min_range_low: uint32,
    pub axis_min_range_high: uint32,
    pub axis_max_range_low: uint32,
    pub axis_max_range_high: uint32,
}

pub struct S {
    pub sensor_id: uint32,
}

impl S {
    pub open spec fn sensor_exists(self, sensor_id: uint32) -> bool;
    pub open spec fn sensor_supports_axis(self, sensor_id: uint32) -> bool;
    pub open spec fn sensor_axis_desc_index(self, sensor_id: uint32) -> uint32;
    pub open spec fn sensor_axis_desc_count(self, sensor_id: uint32) -> uint32;
    pub open spec fn sensor_axis_desc_id(self, sensor_id: uint32, idx: int) -> uint32;
    pub open spec fn sensor_axis_desc_attr_low(self, sensor_id: uint32, idx: int) -> uint32;
    pub open spec fn sensor_axis_desc_attr_high(self, sensor_id: uint32, idx: int) -> uint32;
    pub open spec fn sensor_axis_desc_name(self, sensor_id: uint32, idx: int) -> Seq<char>;
    pub open spec fn sensor_axis_desc_res(self, sensor_id: uint32, idx: int) -> uint32;
    pub open spec fn sensor_axis_desc_min_range_low(self, sensor_id: uint32, idx: int) -> uint32;
    pub open spec fn sensor_axis_desc_min_range_high(self, sensor_id: uint32, idx: int) -> uint32;
    pub open spec fn sensor_axis_desc_max_range_low(self, sensor_id: uint32, idx: int) -> uint32;
    pub open spec fn sensor_axis_desc_max_range_high(self, sensor_id: uint32, idx: int) -> uint32;
}

} // verus!
