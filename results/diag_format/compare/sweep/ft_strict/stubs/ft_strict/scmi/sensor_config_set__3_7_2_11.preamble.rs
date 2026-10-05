use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt16 = u16;
pub type Int5 = i8;
pub type Bool = bool;
pub type Int32 = i32;

pub struct SensorInfo {
    pub enabled: bool,
    pub timestamp_enabled: bool,
    pub update_interval: int,
}

pub struct S {
    pub sensors: Map<UInt32, SensorInfo>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_FOUND: Int32 = -3;
pub const result: Int32 = 100;

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn IsValidSensorConfig(s: S, sensor_id: UInt32, sensor_config: [UInt32; 2]) -> bool;

pub open spec fn SensorSupportsConfig(s: S, sensor_id: UInt32, sensor_config: [UInt32; 2]) -> bool;

pub open spec fn SensorAt(s: S, sensor_id: UInt32) -> SensorInfo;

pub open spec fn Bits(sensor_config: [UInt32; 2], hi: int, lo: int) -> int;

pub open spec fn UpdateIntervalUnchanged(s: S, sensor_id: UInt32) -> bool;

pub open spec fn RequestedInterval(sec: int, exponent: int) -> int;

pub open spec fn ClosestSupportedInterval(s: S, sensor_id: UInt32, requested: int) -> int;

pub open spec fn RoundUpSupportedInterval(s: S, sensor_id: UInt32, requested: int) -> int;

pub open spec fn RoundDownSupportedInterval(s: S, sensor_id: UInt32, requested: int) -> int;

} // verus!
