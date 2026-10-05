use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;
pub type SensorId = u32;

pub struct S {
    pub current_sensor_id: SensorId,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;

pub open spec fn sensor_id(s: S) -> SensorId;

pub open spec fn SensorExists(id: SensorId) -> bool;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub open spec fn IsValidSensorExtendedName(id: SensorId, name: [UInt8; 64]) -> bool;

pub open spec fn IsNullTerminatedUtf8String(name: [UInt8; 64], max_len: int) -> bool;

} // verus!
