use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type RsiCommandReturnCode = u32;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub struct SensorReading {
    pub axis: u64,
    pub value: u64,
}

pub struct S {
    pub agent_id: u64,
    pub sensor_id: u64,
    pub readings: Seq<SensorReading>,
}

pub open spec fn AgentRequestedContinuousUpdates(s: S, agent_id: u64, sensor_id: u64) -> bool;

pub open spec fn SensorIsEnabled(s: S, sensor_id: u64) -> bool;

pub open spec fn IsScalarSensor(s: S, sensor_id: u64) -> bool;

pub open spec fn IsAxisSensor(s: S, sensor_id: u64) -> bool;

pub open spec fn Len(readings: Seq<SensorReading>) -> nat;

pub open spec fn NumSensorAxes(s: S, sensor_id: u64) -> nat;

pub open spec fn AllAxesReportedInOrder(readings: Seq<SensorReading>) -> bool;

pub open spec fn LatestSensorValues(s: S, sensor_id: u64) -> Seq<SensorReading>;

pub open spec fn NotificationInterval(s: S, sensor_id: u64) -> int;

pub open spec fn ConfiguredUpdatePeriod(s: S, sensor_id: u64) -> int;

} // verus!
