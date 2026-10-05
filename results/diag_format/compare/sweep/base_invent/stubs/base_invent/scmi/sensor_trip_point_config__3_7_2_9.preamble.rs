use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub struct TripPointInfo {
    pub count: int,
}

pub struct S {
    pub sensor_count: int,
    pub sensor_id: int,
    pub trip_point_id: int,
    pub trip_point_ev_ctrl: int,
    pub trip_point_val_low: int,
    pub trip_point_val_high: int,
    pub sensor_trip_points: Seq<TripPointInfo>,
}

} // verus!
