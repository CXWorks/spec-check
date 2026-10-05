use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;

pub const NOT_SUPPORTED: i32 = -1;

pub const INVALID_PARAMETERS: i32 = -2;

pub const NOT_FOUND: i32 = -4;

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorNumTripPoints(s: S, sensor_id: UInt32) -> UInt32;

pub open spec fn SensorSupportsTripPoints(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorTripPointEventCtrl(s: S, sensor_id: UInt32, trip_point_id: int) -> UInt32;

pub open spec fn SensorTripPointValue(s: S, sensor_id: UInt32, trip_point_id: int) -> UInt64;

pub open spec fn TripPointNotifyEnabled(s: S, sensor_id: UInt32) -> bool;

} // verus!
