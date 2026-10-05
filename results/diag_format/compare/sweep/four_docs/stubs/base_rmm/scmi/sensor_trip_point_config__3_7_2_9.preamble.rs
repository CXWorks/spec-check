use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub struct TripPointState {
    pub value: UInt64,
    pub ev_ctrl: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_FOUND: Int32 = -4;

pub const sensor_id: UInt32 = 0;
pub const trip_point_id: UInt32 = 0;
pub const trip_point_ev_ctrl: UInt32 = 0;
pub const trip_point_val_low: UInt64 = 0;
pub const trip_point_val_high: UInt64 = 0;

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn NumTripPoints(s: S, sensor_id: UInt32) -> UInt32;

pub open spec fn AreLegalTripPointParams(s: S, sensor_id: UInt32, ev_ctrl: UInt32, val_low: UInt64, val_high: UInt64) -> bool;

pub open spec fn SensorSupportsTripPointEvents(s: S, sensor_id: UInt32) -> bool;

pub open spec fn TripPoint(s: S, sensor_id: UInt32, trip_point_id: UInt32) -> TripPointState;

} // verus!
