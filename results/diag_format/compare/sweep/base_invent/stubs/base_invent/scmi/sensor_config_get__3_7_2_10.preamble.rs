use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub type SensorId = u32;

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = 1;

pub const sensor_id: SensorId = 0;

pub struct S {
    pub sensors: Seq<SensorId>,
}

pub open spec fn SensorExists(s: S, id: SensorId) -> bool;

} // verus!
