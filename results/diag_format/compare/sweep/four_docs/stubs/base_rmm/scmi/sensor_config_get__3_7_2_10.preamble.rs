use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type SensorId = u32;

pub struct Sensor {
    pub update_interval: u32,
    pub timestamped: bool,
    pub enabled: bool,
}

pub struct S {
    pub current_sensor_id: SensorId,
    pub sensors: Seq<Sensor>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;

pub open spec fn sensor_id(s: S) -> SensorId;

pub open spec fn SensorExists(id: SensorId) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn SensorAt(s: S, id: SensorId) -> Sensor;

pub open spec fn SensorSupportsUpdateInterval(s: S, id: SensorId) -> bool;

} // verus!
