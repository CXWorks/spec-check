use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub open spec fn SensorTripPointNotificationRequested(s: S, sensor_id: u32, trip_point: int) -> bool;

} // verus!
