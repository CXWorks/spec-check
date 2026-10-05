use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = (-4int) as i32;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;

pub spec const result: bool = arbitrary();
pub spec const agent: AgentId = arbitrary();

pub uninterp spec fn IsValidSensor(s: S, sensor_id: UInt32) -> bool;

pub uninterp spec fn SensorSupportsContinuousUpdateNotify(s: S, sensor_id: UInt32) -> bool;

pub uninterp spec fn IsSupportedNotifyEnable(s: S, notify_enable: UInt32) -> bool;

pub uninterp spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub uninterp spec fn SensorUpdateNotifyEnabled(s: S, sensor_id: UInt32, agent: AgentId) -> bool;

pub uninterp spec fn Bits(value: UInt32, hi: int, lo: int) -> int;

} // verus!
