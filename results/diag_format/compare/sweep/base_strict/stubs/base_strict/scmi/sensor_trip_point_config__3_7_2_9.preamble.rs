use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct TripPointState {
    pub ev_ctrl: UInt32,
    pub value: int,
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn SensorExists(sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn Bits(x: UInt32, hi: nat, lo: nat) -> UInt32;

pub open spec fn NumTripPoints(sensor_id: UInt32) -> UInt32;

pub open spec fn TripPointConfigParamsValid(sensor_id: UInt32, trip_point_ev_ctrl: UInt32, trip_point_val_low: UInt32, trip_point_val_high: UInt32) -> bool;

pub open spec fn SensorSupportsTripPointEvents(sensor_id: UInt32) -> bool;

pub open spec fn TripPoint(s: S, trip_point_id: UInt32) -> TripPointState;

pub open spec fn TripPointNotifyEnabled(sensor_id: UInt32) -> bool;

pub open spec fn GeneratesTripPointEventOnCrossing(sensor_id: UInt32, trip_point_id: UInt32) -> bool;

} // verus!
