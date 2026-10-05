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
pub open spec fn SensorEventControlIsValid(sensor_event_control: u32) -> bool;
pub open spec fn SensorTripPointNotifySupported(s: S, sensor_id: u32) -> bool;
pub open spec fn SensorTripPointNotifyEnabled(s: S, sensor_id: u32) -> bool;
pub open spec fn SensorTripPointConfigsUnchanged(old_s: S, new_s: S) -> bool;
pub open spec fn SensorTripPointNotifyUnchangedExcept(old_s: S, new_s: S, sensor_id: u32) -> bool;

} // verus!
