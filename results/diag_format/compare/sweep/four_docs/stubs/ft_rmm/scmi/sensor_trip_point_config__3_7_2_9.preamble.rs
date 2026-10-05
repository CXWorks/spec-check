use vstd::prelude::*;
verus! {

pub type UInt2 = u8;
pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub struct TripPointRec {
    pub value: UInt32,
    pub ev_ctrl: UInt2,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn SensorExists(s: S, sensor_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn NumTripPoints(s: S, sensor_id: UInt32) -> UInt8;

pub open spec fn AreLegalTripPointParams(s: S, sensor_id: UInt32, trip_point_ev_ctrl: UInt2, trip_point_val_low: UInt32, trip_point_val_high: UInt32) -> bool;

pub open spec fn SensorSupportsTripPointEvents(s: S, sensor_id: UInt32) -> bool;

pub open spec fn TripPoint(s: S, sensor_id: UInt32, trip_point_id: int) -> TripPointRec;

} // verus!
