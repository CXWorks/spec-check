use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct SENSOR_READING {
    pub value_low: UInt32,
    pub value_high: UInt32,
    pub timestamp_low: UInt32,
    pub timestamp_high: UInt32,
}

pub struct SensorInfo {
    pub enabled: bool,
    pub pending_reading_complete: bool,
}

pub struct S {
    pub sensors: Seq<SensorInfo>,
}

pub struct Array<T> {
    pub data: Seq<T>,
}

impl<T> Array<T> {
    pub open spec fn spec_index(self, i: UInt32) -> T;
}

pub const sensor_id: UInt32 = 0;
pub const flags: UInt32 = 0;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const PROTOCOL_ERROR: Int32 = -3;
pub const HARDWARE_ERROR: Int32 = -4;

pub open spec fn SensorExists(s: S, id: UInt32) -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn IsValidReadingFlags(s: S, f: UInt32) -> bool;
pub open spec fn SensorIsEnabled(s: S, id: UInt32) -> bool;
pub open spec fn Bits(s: S, v: UInt32, hi: UInt32, lo: UInt32) -> UInt32;
pub open spec fn SensorHardwareFault(s: S, id: UInt32) -> bool;
pub open spec fn ReadingsMatchCurrentSensorValue(readings: Array<SENSOR_READING>, id: UInt32) -> bool;
pub open spec fn SensorReadingCompleteEnqueued(s: S, id: UInt32) -> bool;
pub open spec fn SensorIsScalar(s: S, id: UInt32) -> bool;
pub open spec fn ReadingCount(readings: Array<SENSOR_READING>) -> UInt32;
pub open spec fn SensorAxisCount(s: S, id: UInt32) -> UInt32;
pub open spec fn ReadingsInAxisOrder(readings: Array<SENSOR_READING>, id: UInt32) -> bool;
pub open spec fn TimestampCollected(s: S, id: UInt32) -> bool;
pub open spec fn SensorAt(s: S, id: UInt32) -> SensorInfo;

} // verus!
