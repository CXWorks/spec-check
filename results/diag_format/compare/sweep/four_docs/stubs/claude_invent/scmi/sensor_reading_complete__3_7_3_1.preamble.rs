use vstd::prelude::*;
verus! {

pub struct SENSOR_READING {
    pub axis: u32,
    pub value: i64,
    pub timestamp: u64,
}

pub struct S {
    pub sensors: Seq<u32>,
    pub fault_flags: Seq<bool>,
}

pub const SUCCESS: i32 = 0;
pub const HARDWARE_ERROR: i32 = 1;

pub open spec fn SensorHasHardwareFault(s: S, sensor_id: u32) -> bool;

pub open spec fn SensorNumAxes(s: S, sensor_id: u32) -> nat;

pub open spec fn SensorAxisReading(s: S, sensor_id: u32, i: int) -> SENSOR_READING;

} // verus!
