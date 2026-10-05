use vstd::prelude::*;

verus! {

pub open spec fn TripPointEventRequested(sensor_id: u32, trip_point_desc: int) -> bool;

pub open spec fn SensorCrossedOrReachedTripPoint(sensor_id: u32, trip_point_desc: int) -> bool;

pub open spec fn TripPointCrossedPositive(sensor_id: u32, trip_point_desc: int) -> bool;

pub open spec fn TripPointCrossedNegative(sensor_id: u32, trip_point_desc: int) -> bool;

} // verus!
