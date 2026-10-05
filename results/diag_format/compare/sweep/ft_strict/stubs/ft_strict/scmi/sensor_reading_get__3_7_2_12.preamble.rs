use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type Result = i32;

pub struct SENSOR_READING {
    pub value_low: u32,
    pub value_high: u32,
    pub timestamp_low: u32,
    pub timestamp_high: u32,
}

pub struct SensorState {
    pub pending_reading_complete: bool,
}

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: Result = 0;
pub const NOT_FOUND: Result = -1;
pub const INVALID_PARAMETERS: Result = -2;
pub const PROTOCOL_ERROR: Result = -3;
pub const HARDWARE_ERROR: Result = -4;

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;
pub open spec fn ResultEqual(status: Int32, r: Result) -> bool;
pub open spec fn IsValidReadingFlags(s: S, flags: UInt32) -> bool;
pub open spec fn SensorIsEnabled(s: S, sensor_id: UInt32) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn SensorHardwareFault(s: S, sensor_id: UInt32) -> bool;
pub open spec fn ReadingsMatchCurrentSensorValue(s: S, readings: [SENSOR_READING; 4], sensor_id: UInt32) -> bool;
pub open spec fn SensorReadingCompleteEnqueued(s: S, sensor_id: UInt32) -> bool;
pub open spec fn SensorIsScalar(s: S, sensor_id: UInt32) -> bool;
pub open spec fn ReadingCount(s: S, readings: [SENSOR_READING; 4]) -> UInt32;
pub open spec fn SensorAxisCount(s: S, sensor_id: UInt32) -> UInt32;
pub open spec fn ReadingsInAxisOrder(s: S, readings: [SENSOR_READING; 4], sensor_id: UInt32) -> bool;
pub open spec fn TimestampCollected(s: S, sensor_id: UInt32) -> bool;
pub open spec fn SensorAt(s: S, sensor_id: UInt32) -> SensorState;

} // verus!
