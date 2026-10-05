use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub sensor_id_field: u32,
    pub notify_enable_field: u32,
    pub agent_field: u32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1i32;
pub spec const INVALID_PARAMETERS: Int32 = -2i32;
pub spec const NOT_FOUND: Int32 = -4i32;

pub open spec fn sensor_id(s: S) -> u32;

pub open spec fn notify_enable(s: S) -> u32;

pub open spec fn agent(s: S) -> u32;

pub open spec fn IsValidSensor(s: S, sensor_id: u32) -> bool;

pub open spec fn SensorSupportsContinuousUpdateNotify(s: S, sensor_id: u32) -> bool;

pub open spec fn IsSupportedNotifyEnable(s: S, value: int) -> bool;

pub open spec fn Bits(value: u32, high: int, low: int) -> int;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn SensorUpdateNotifyEnabled(s: S, sensor_id: u32, agent: u32) -> bool;

} // verus!
