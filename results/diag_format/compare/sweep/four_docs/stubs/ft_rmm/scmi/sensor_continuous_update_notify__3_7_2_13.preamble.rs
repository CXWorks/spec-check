use vstd::prelude::*;

verus! {

pub type uint32 = u32;
pub type int32 = i32;

pub struct S {
    pub sensor_count: nat,
    pub notify_state: Map<(uint32, uint32), bool>,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -1;
pub const NOT_SUPPORTED: int32 = -2;
pub const INVALID_PARAMETERS: int32 = -3;

#[allow(non_upper_case_globals)]
pub spec const result: int32 = (-100int) as int32;

pub open spec fn IsValidSensorId(s: S, sensor_id: uint32) -> bool;

pub open spec fn SensorSupportsContinuousUpdateNotify(s: S, sensor_id: uint32) -> bool;

pub open spec fn IsValidNotifyEnable(s: S, notify_enable: uint32) -> bool;

pub open spec fn ResultEqual(status: int32, code: int32) -> bool;

pub open spec fn SensorUpdateNotifyEnabled(s: S, agent_id: uint32, sensor_id: uint32) -> bool;

} // verus!
