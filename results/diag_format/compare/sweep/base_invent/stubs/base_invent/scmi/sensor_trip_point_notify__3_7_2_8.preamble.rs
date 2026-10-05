use vstd::prelude::*;

verus! {

pub type int32 = i32;

pub type SensorId = u16;

pub type SensorEventControl = u8;

pub const T0: int32 = 0;
pub const NOT_FOUND: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;
pub const NOT_SUPPORTED: int32 = -3;

pub const sensor_id: SensorId = 0;

pub struct S {
    pub sensor_id: SensorId,
    pub sensor_event_control: SensorEventControl,
}

} // verus!
