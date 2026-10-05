use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub enum RmiStatusCode {
    RmiSuccess,
    RmiErrorInput,
    RmiErrorRealm,
    RmiErrorRec,
    RmiErrorRtt,
    RmiErrorDevice,
    RmiErrorNotSupported,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> int;

pub open spec fn TripPointEventRequested(s: S, sensor_id: UInt32, trip_point_id: int) -> bool;

pub open spec fn TripPointReachedOrCrossed(s: S, sensor_id: UInt32, trip_point_id: int) -> bool;

pub open spec fn TripPointCrossedInPositiveDirection(s: S, sensor_id: UInt32, trip_point_id: int) -> bool;

pub open spec fn TripPointCrossedInNegativeDirection(s: S, sensor_id: UInt32, trip_point_id: int) -> bool;

pub open spec fn MultipleTripPointCrossingsMayBeReportedByOneNotification(s: S, sensor_id: UInt32) -> bool;

} // verus!
