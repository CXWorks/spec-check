use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub struct ReadingFlags {
    pub r#async: UInt32,
}

pub struct SensorReading {
    pub value_low: UInt32,
    pub value_high: UInt32,
    pub timestamp_low: UInt32,
    pub timestamp_high: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const PROTOCOL_ERROR: Int32 = -3;
pub const HARDWARE_ERROR: Int32 = -4;

pub spec const sensor_id: UInt32 = 0;
pub spec const flags: ReadingFlags = ReadingFlags { r#async: 0 };
pub spec const readings: Seq<SensorReading> = Seq::<SensorReading>::empty();
pub spec const N: UInt32 = 0;
pub spec const i: int = 0;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;
pub open spec fn IsValidReadingFlags(s: S, flags: ReadingFlags) -> bool;
pub open spec fn SensorIsEnabled(s: S, sensor_id: UInt32) -> bool;
pub open spec fn SensorHasHardwareFault(s: S, sensor_id: UInt32) -> bool;
pub open spec fn CurrentSensorReadings(s: S, sensor_id: UInt32) -> Seq<SensorReading>;
pub open spec fn SensorIsAxial(s: S, sensor_id: UInt32) -> bool;
pub open spec fn SensorNumAxes(s: S, sensor_id: UInt32) -> UInt32;
pub open spec fn SensorCollectsTimestamp(s: S, sensor_id: UInt32) -> bool;
pub open spec fn AsyncReadingRequestEnqueued(s: S, sensor_id: UInt32) -> bool;

} // verus!
