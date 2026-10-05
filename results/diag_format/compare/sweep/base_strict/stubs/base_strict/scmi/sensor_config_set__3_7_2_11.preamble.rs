use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub type SensorId = u32;

pub type SensorConfig = u32;

pub type Interval = int;

pub struct Sensor {
    pub enabled: bool,
    pub timestamp_enabled: bool,
    pub update_interval: Interval,
}

pub struct S {
    pub sensors: Map<SensorId, Sensor>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;
pub const INVALID_PARAMETERS: Int32 = 2;
pub const NOT_SUPPORTED: Int32 = 3;

#[allow(non_upper_case_globals)]
pub const sensor_id: SensorId = 0;

#[allow(non_upper_case_globals)]
pub const sensor_config: SensorConfig = 0;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn SensorExists(s: S, id: SensorId) -> bool;

pub open spec fn IsValidSensorConfig(s: S, id: SensorId, config: SensorConfig) -> bool;

pub open spec fn SensorSupportsConfig(s: S, id: SensorId, config: SensorConfig) -> bool;

pub open spec fn SensorAt(s: S, id: SensorId) -> Sensor;

pub open spec fn Bits(value: SensorConfig, hi: int, lo: int) -> int;

pub open spec fn UpdateIntervalUnchanged(old_s: S, new_s: S, id: SensorId) -> bool;

pub open spec fn RequestedInterval(multiplier: int, exponent: int) -> Interval;

pub open spec fn ClosestSupportedInterval(s: S, id: SensorId, requested: Interval) -> Interval;

pub open spec fn RoundUpSupportedInterval(s: S, id: SensorId, requested: Interval) -> Interval;

pub open spec fn RoundDownSupportedInterval(s: S, id: SensorId, requested: Interval) -> Interval;

} // verus!
