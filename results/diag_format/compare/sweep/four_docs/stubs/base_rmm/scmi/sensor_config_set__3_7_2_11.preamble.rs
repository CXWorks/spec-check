use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub struct SensorConfig {
    pub raw: u32,
}

impl SensorConfig {
    pub open spec fn spec_index(self, i: int) -> u32;
}

pub struct Sensor {
    pub enabled: u32,
    pub timestamp_enabled: u32,
    pub update_interval: u32,
}

pub struct S {
    pub sensor_id: u8,
    pub sensor_config: SensorConfig,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;
pub const INVALID_PARAMETERS: Int32 = 2;
pub const NOT_SUPPORTED: Int32 = 3;

pub open spec fn sensor_id(s: S) -> UInt8;

pub open spec fn sensor_config(s: S) -> SensorConfig;

pub open spec fn IsExistingSensor(id: UInt8) -> bool;

pub open spec fn AreValidSensorConfigParameters(config: SensorConfig) -> bool;

pub open spec fn IsSensorConfigSupported(id: UInt8, config: SensorConfig) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn SensorAt(s: S, id: UInt8) -> Sensor;

pub open spec fn RoundedUpdateInterval(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32;

} // verus!
