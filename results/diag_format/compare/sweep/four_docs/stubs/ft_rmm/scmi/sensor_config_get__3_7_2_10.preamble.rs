use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;

#[is_variant]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub enum RmiStatusCode {
    Success,
    NotFound,
    Error,
}

pub struct Sensor {
    pub update_interval: UInt32,
    pub timestamped: bool,
    pub enabled: bool,
}

pub struct S {
    pub sensors: Map<UInt32, Sensor>,
}

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorAt(s: S, sensor_id: UInt32) -> Sensor;

pub open spec fn SensorSupportsUpdateInterval(s: S, sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

} // verus!
