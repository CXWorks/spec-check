use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub struct S {
    pub current_sensor: UInt32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = 1;

pub open spec fn sensor_id(s: S) -> UInt32;

pub open spec fn SensorExists(id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn SensorExtendedName(id: UInt32) -> [UInt8; 64];

pub open spec fn IsNullTerminatedUtf8(name: [UInt8; 64], len: int) -> bool;

} // verus!
