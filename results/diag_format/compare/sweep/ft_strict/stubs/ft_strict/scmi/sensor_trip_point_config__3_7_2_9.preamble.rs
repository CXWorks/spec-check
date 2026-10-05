use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub struct TripPointInfo {
    pub ev_ctrl: UInt32,
    pub value: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2) as i32;
pub spec const NOT_FOUND: Int32 = (-4) as i32;
#[allow(non_upper_case_globals)]
pub spec const result: Int32 = (-100) as i32;

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn NumTripPoints(s: S, sensor_id: UInt32) -> UInt32;
pub open spec fn TripPointConfigParamsValid(s: S, sensor_id: UInt32, trip_point_ev_ctrl: UInt32, trip_point_val_low: UInt32, trip_point_val_high: UInt32) -> bool;
pub open spec fn SensorSupportsTripPointEvents(s: S, sensor_id: UInt32) -> bool;
pub open spec fn TripPoint(s: S, sensor_id: UInt32, trip_point_id: UInt32) -> TripPointInfo;
pub open spec fn TripPointNotifyEnabled(s: S, sensor_id: UInt32) -> bool;
pub open spec fn GeneratesTripPointEventOnCrossing(s: S, sensor_id: UInt32, trip_point_id: UInt32) -> bool;

} // verus!
