use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type SensorValue = i64;

pub struct SENSOR_READING {
    pub count: UInt32,
    pub values: Seq<SensorValue>,
}

pub struct S {
    pub sensor_state: int,
    pub pending_requests: int,
}

pub enum Field {
    SensorState,
    PendingRequests,
}

pub const SUCCESS: Int32 = 0;
pub const HARDWARE_ERROR: Int32 = 1;

pub open spec fn SensorHasHardwareFault(sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsResponseToAsyncSensorReadingGet(sensor_id: UInt32) -> bool;

pub open spec fn ReadingCount(readings: SENSOR_READING) -> UInt32;

pub open spec fn SensorAxisCount(sensor_id: UInt32) -> UInt32;

pub open spec fn ReadingAt(readings: SENSOR_READING, i: UInt32) -> SensorValue;

pub open spec fn SensorAxisReading(sensor_id: UInt32, i: UInt32) -> SensorValue;

pub open spec fn FieldIsModified(field: Field, old_s: S, new_s: S) -> bool;

} // verus!
