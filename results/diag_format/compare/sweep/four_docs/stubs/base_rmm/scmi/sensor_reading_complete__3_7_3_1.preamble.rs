use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct SensorAxisReading {
    pub value: int32,
}

pub type SENSOR_READING = Seq<SensorAxisReading>;

pub struct S {
    pub dummy: int,
}

pub const HARDWARE_ERROR: int32 = 5;

pub open spec fn SensorHasHardwareFault(sensor_id: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn Length(readings: SENSOR_READING) -> nat;

pub open spec fn NumSensorAxes(sensor_id: uint32) -> nat;

pub open spec fn ReadingsReportedInAxisOrder(readings: SENSOR_READING, sensor_id: uint32) -> bool;

} // verus!
