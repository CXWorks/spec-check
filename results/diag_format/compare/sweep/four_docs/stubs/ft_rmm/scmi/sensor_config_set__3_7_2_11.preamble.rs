use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;

pub struct SensorConfigBits {
    pub raw: u16,
}

impl SensorConfigBits {
    pub open spec fn spec_index(self, i: int) -> u16;
}

pub type UInt16 = SensorConfigBits;

pub struct Sensor {
    pub enabled: u16,
    pub timestamp_enabled: u16,
    pub update_interval: u64,
}

pub struct S {
    pub sensors: Map<UInt32, Sensor>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;
pub const INVALID_PARAMETERS: Int32 = 2;
pub const NOT_SUPPORTED: Int32 = 3;

pub open spec fn IsExistingSensor(s: S, sensor_id: UInt32) -> bool;

pub open spec fn AreValidSensorConfigParameters(s: S, sensor_config: UInt16) -> bool;

pub open spec fn IsSensorConfigSupported(s: S, sensor_id: UInt32, sensor_config: UInt16) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn ResultNotEqual(a: Int32, b: Int32) -> bool;

pub open spec fn SensorAt(s: S, sensor_id: UInt32) -> Sensor;

pub open spec fn RoundedUpdateInterval(s: S, mantissa: u16, exponent: u16, round_up: u16, round_down: u16) -> u64;

} // verus!
