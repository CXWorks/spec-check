use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub sensor_count: nat,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = 1;

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn SensorExtendedName(s: S, sensor_id: UInt32) -> [UInt8; 64];

pub open spec fn IsNullTerminatedUtf8(name: [UInt8; 64], len: int) -> bool;

} // verus!
