use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub struct Flags {
    pub r#async: u32,
}

pub struct SENSOR_READING {
    pub value_low: u32,
    pub value_high: u32,
    pub timestamp_low: u32,
    pub timestamp_high: u32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = -1;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const PROTOCOL_ERROR: Int32 = -3;
pub spec const HARDWARE_ERROR: Int32 = -4;

pub spec const result: Int32 = -100;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;

pub open spec fn IsValidReadingFlags(s: S, flags: Flags) -> bool;

pub open spec fn SensorIsEnabled(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorHasHardwareFault(s: S, sensor_id: UInt32) -> bool;

pub open spec fn CurrentSensorReadings(s: S, sensor_id: UInt32) -> [SENSOR_READING; 1];

pub open spec fn SensorIsAxial(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorNumAxes(s: S, sensor_id: UInt32) -> u32;

pub open spec fn SensorCollectsTimestamp(s: S, sensor_id: UInt32) -> bool;

pub open spec fn AsyncReadingRequestEnqueued(s: S, sensor_id: UInt32) -> bool;

} // verus!
