use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -1;
pub const NOT_SUPPORTED: i32 = -2;
pub const INVALID_PARAMETERS: i32 = -3;

pub open spec fn SensorIdValid(s: S, sensor_id: u32) -> bool;

pub open spec fn SensorSupportsContinuousUpdateNotify(s: S, sensor_id: u32) -> bool;

pub open spec fn SensorUpdateNotifyEnabled(s: S, agent_id: u32, sensor_id: u32) -> bool;

} // verus!
