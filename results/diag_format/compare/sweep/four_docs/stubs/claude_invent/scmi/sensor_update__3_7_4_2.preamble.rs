use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsSensorEnabled(s: S, sensor_id: UInt32) -> bool;

pub open spec fn IsSensorContinuousUpdateNotifyRequested(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorReportsAxes(s: S, sensor_id: UInt32) -> bool;

pub open spec fn SensorNumAxes(s: S, sensor_id: UInt32) -> UInt32;

} // verus!
