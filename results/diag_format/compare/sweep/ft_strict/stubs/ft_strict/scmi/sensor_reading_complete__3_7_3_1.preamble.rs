use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub struct SENSOR_READING {
    pub value: i64,
}

pub struct S {
    pub sensor_count: UInt32,
}

pub enum RmiStatusCode {
    Success,
    HardwareError,
    InvalidParameter,
}

pub spec const SUCCESS: Int32 = 0;

pub spec const HARDWARE_ERROR: Int32 = 1;

pub open spec fn SensorHasHardwareFault(s: S, sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsResponseToAsyncSensorReadingGet(s: S, sensor_id: UInt32) -> bool;

pub open spec fn ReadingCount(s: S, readings: [SENSOR_READING; 3]) -> UInt32;

pub open spec fn SensorAxisCount(s: S, sensor_id: UInt32) -> UInt32;

pub open spec fn ReadingAt(s: S, readings: [SENSOR_READING; 3], i: UInt32) -> SENSOR_READING;

pub open spec fn SensorAxisReading(s: S, sensor_id: UInt32, i: UInt32) -> SENSOR_READING;

} // verus!
