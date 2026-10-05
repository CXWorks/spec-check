use vstd::prelude::*;

verus! {

pub type UInt = u64;
pub type int32 = i32;
pub type uint32 = u32;

pub struct SENSOR_READING {
    pub data: Seq<int>,
}

pub struct S {
    pub sensor_faults: Map<uint32, bool>,
    pub sensor_axes: Map<uint32, int>,
}

pub const HARDWARE_ERROR: int32 = -5;

pub open spec fn SensorHasHardwareFault(s: S, sensor_id: uint32) -> bool;

pub open spec fn ResultEqual(status: int32, code: int32) -> bool;

pub open spec fn Length(readings: SENSOR_READING) -> int;

pub open spec fn NumSensorAxes(s: S, sensor_id: uint32) -> int;

pub open spec fn ReadingsReportedInAxisOrder(readings: SENSOR_READING, sensor_id: uint32) -> bool;

} // verus!
