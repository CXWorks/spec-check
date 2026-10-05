use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = 1;
pub spec const NOT_SUPPORTED: Int32 = 2;

pub open spec fn IsValidSensor(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorReportsAxisValues(s: S, sensor_id: UInt32) -> bool;

} // verus!
