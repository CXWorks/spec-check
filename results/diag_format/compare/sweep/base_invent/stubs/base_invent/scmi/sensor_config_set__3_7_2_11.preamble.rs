use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub sensor_count: nat,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_FOUND: int32 = -4i32;
pub spec const INVALID_PARAMETERS: int32 = -2i32;
pub spec const NOT_SUPPORTED: int32 = -1i32;

pub spec const sensor_id: uint32 = 1;
pub spec const sensor_update_interval: int32 = 2;
pub spec const timestamp_reporting: uint32 = 3;
pub spec const sensor_state: uint32 = 4;

pub open spec fn SensorExists(s: S, id: uint32) -> bool;

pub open spec fn SensorConfigSet(old_s: S, new_s: S, id: uint32, update_interval: int32, ts_reporting: uint32, state: uint32) -> bool;

} // verus!
