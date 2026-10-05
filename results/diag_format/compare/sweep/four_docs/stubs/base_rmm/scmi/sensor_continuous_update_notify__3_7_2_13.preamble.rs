use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint8 = u8;
pub type uint16 = u16;

pub struct Agent {
    pub id: u64,
}

pub struct S {
    pub agent: Agent,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = 1;
pub const NOT_SUPPORTED: int32 = 2;
pub const INVALID_PARAMETERS: int32 = 3;

pub spec const sensor_id: uint16 = 0;
pub spec const notify_enable: Seq<uint8> = Seq::empty();
pub spec const agent: Agent = Agent { id: 0 };

pub uninterp spec fn IsValidSensorId(sid: uint16) -> bool;
pub uninterp spec fn SensorSupportsContinuousUpdateNotify(sid: uint16) -> bool;
pub uninterp spec fn IsValidNotifyEnable(ne: Seq<uint8>) -> bool;
pub uninterp spec fn ResultEqual(a: int32, b: int32) -> bool;
pub uninterp spec fn SensorUpdateNotifyEnabled(ag: Agent, sid: uint16) -> bool;

} // verus!
