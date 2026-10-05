use vstd::prelude::*;

verus! {

pub type uint32 = u32;

pub enum RmiStatusCode {
    RmiSuccess,
    RmiErrorInput,
    RmiErrorRealm,
    RmiErrorRec,
    RmiErrorRtt,
    RmiErrorDevice,
    RmiErrorNotSupported,
}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool {
        self is Ok
    }

    pub open spec fn is_Err(self) -> bool {
        self is Err
    }
}

pub struct S {
    pub dummy: nat,
}

pub open spec fn TripPointEventRequested(s: S, sensor_id: uint32, trip_point_id: uint32) -> bool;

pub open spec fn SensorCrossedOrReachedTripPoint(s: S, sensor_id: uint32, trip_point_id: uint32) -> bool;

pub open spec fn TripPointCrossedPositive(s: S, sensor_id: uint32, trip_point_id: uint32) -> bool;

pub open spec fn TripPointCrossedNegative(s: S, sensor_id: uint32, trip_point_id: uint32) -> bool;

} // verus!
