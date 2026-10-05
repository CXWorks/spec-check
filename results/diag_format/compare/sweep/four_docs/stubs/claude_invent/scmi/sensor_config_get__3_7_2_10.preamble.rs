use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0i32;

pub const NOT_FOUND: Int32 = 1i32;

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorUpdateIntervalSec(s: S, sensor_id: UInt32) -> u32;

pub open spec fn SensorUpdateIntervalExponent(s: S, sensor_id: UInt32) -> u32;

pub open spec fn SensorSupportsUpdateInterval(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorTimestampEnabled(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorEnabled(s: S, sensor_id: UInt32) -> bool;

} // verus!
