use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = 1;

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorExtendedNameSupported(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorExtendedName(s: S, sensor_id: UInt32) -> Seq<u8>;

} // verus!
