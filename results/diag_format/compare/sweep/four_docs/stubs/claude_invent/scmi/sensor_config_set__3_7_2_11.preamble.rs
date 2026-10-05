use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = 1;
pub const INVALID_PARAMETERS: i32 = 2;
pub const NOT_SUPPORTED: i32 = 3;

pub open spec fn SensorExists(s: S, sensor_id: u32) -> bool;
pub open spec fn SensorConfigParamsInvalid(s: S, sensor_id: u32, sensor_config: u32) -> bool;
pub open spec fn SensorConfigSupported(s: S, sensor_id: u32, sensor_config: u32) -> bool;
pub open spec fn SensorEnabled(s: S, sensor_id: u32) -> bool;
pub open spec fn SensorTimestampEnabled(s: S, sensor_id: u32) -> bool;
pub open spec fn SensorUpdateInterval(s: S, sensor_id: u32) -> int;
pub open spec fn SensorUpdateIntervalConfigured(old_s: S, new_s: S, sensor_id: u32, sec: u32, mult: u32, round_up: bool, round_down: bool) -> bool;
pub open spec fn OtherSensorsUnchanged(old_s: S, new_s: S, sensor_id: u32) -> bool;

} // verus!
