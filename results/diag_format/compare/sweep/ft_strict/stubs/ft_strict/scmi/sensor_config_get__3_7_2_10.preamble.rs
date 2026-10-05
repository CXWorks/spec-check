use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn SensorUpdateIntervalSupported(s: S, sensor_id: UInt32) -> bool;
pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> int;
pub open spec fn SensorUpdateIntervalSec(s: S, sensor_id: UInt32) -> int;
pub open spec fn SensorUpdateIntervalExponent(s: S, sensor_id: UInt32) -> int;
pub open spec fn SensorIsTimestamped(s: S, sensor_id: UInt32) -> bool;
pub open spec fn SensorIsEnabled(s: S, sensor_id: UInt32) -> bool;

} // verus!
