use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    RsiError,
    RsiErrorInput,
    RsiErrorUnknown,
}

pub struct S {
    pub dummy: int,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn TripPointEventRequested(s: S, sensor_id: UInt32, trip_point_id: UInt32) -> bool;

pub open spec fn TripPointReachedOrCrossed(s: S, sensor_id: UInt32, trip_point_id: UInt32) -> bool;

pub open spec fn TripPointCrossedInPositiveDirection(s: S, sensor_id: UInt32, trip_point_id: UInt32) -> bool;

pub open spec fn TripPointCrossedInNegativeDirection(s: S, sensor_id: UInt32, trip_point_id: UInt32) -> bool;

pub open spec fn MultipleTripPointCrossingsMayBeReportedByOneNotification(s: S, sensor_id: UInt32) -> bool;

} // verus!
