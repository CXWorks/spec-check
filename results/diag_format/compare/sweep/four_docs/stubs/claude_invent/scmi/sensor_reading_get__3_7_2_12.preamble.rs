use vstd::prelude::*;

verus! {

pub struct SensorReading {
    pub value: i64,
    pub timestamp: u64,
}

pub struct S {
    pub sensors: Map<u32, nat>,
    pub pending: Set<u32>,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = 1;
pub const INVALID_PARAMETERS: i32 = 2;
pub const PROTOCOL_ERROR: i32 = 3;
pub const HARDWARE_ERROR: i32 = 4;

pub open spec fn SensorExists(s: S, sensor_id: u32) -> bool;

pub open spec fn SensorReadingFlagsInvalid(s: S, sensor_id: u32, flags: u32) -> bool;

pub open spec fn SensorEnabled(s: S, sensor_id: u32) -> bool;

pub open spec fn SensorHardwareFault(s: S, sensor_id: u32) -> bool;

pub open spec fn SensorHasAxes(s: S, sensor_id: u32) -> bool;

pub open spec fn SensorNumAxes(s: S, sensor_id: u32) -> u32;

pub open spec fn SensorReadingMatches(s: S, sensor_id: u32, i: int, r: SensorReading) -> bool;

pub open spec fn AsyncSensorReadPending(s: S, sensor_id: u32) -> bool;

} // verus!
