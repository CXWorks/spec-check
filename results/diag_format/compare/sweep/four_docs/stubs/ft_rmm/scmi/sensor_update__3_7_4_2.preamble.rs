use vstd::prelude::*;

verus! {

pub type uint32 = u32;

pub struct SENSOR_READING {
    pub value: int,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn AgentRequestedContinuousUpdates(s: S, agent_id: uint32, sensor_id: uint32) -> bool;

pub open spec fn SensorIsEnabled(s: S, sensor_id: uint32) -> bool;

pub open spec fn IsScalarSensor(s: S, sensor_id: uint32) -> bool;

pub open spec fn IsAxisSensor(s: S, sensor_id: uint32) -> bool;

pub open spec fn Len(s: S, readings: [SENSOR_READING; 1]) -> int;

pub open spec fn NumSensorAxes(s: S, sensor_id: uint32) -> int;

pub open spec fn AllAxesReportedInOrder(s: S, readings: [SENSOR_READING; 1]) -> bool;

pub open spec fn LatestSensorValues(s: S, sensor_id: uint32) -> [SENSOR_READING; 1];

pub open spec fn NotificationInterval(s: S, sensor_id: uint32) -> int;

pub open spec fn ConfiguredUpdatePeriod(s: S, sensor_id: uint32) -> int;

} // verus!
