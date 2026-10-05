use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type SensorId = u32;
pub type AgentId = u32;

pub struct SENSOR_READING {
    pub value: int,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn SensorEnabled(s: S, sensor_id: UInt32) -> bool;
pub open spec fn RecipientAgent() -> AgentId;
pub open spec fn ContinuousUpdateRequested(s: S, agent: AgentId, sensor_id: UInt32) -> bool;
pub open spec fn IsScalarSensor(s: S, sensor_id: UInt32) -> bool;
pub open spec fn IsAxisSensor(s: S, sensor_id: UInt32) -> bool;
pub open spec fn NumReadings(s: S, readings: [SENSOR_READING; 1]) -> int;
pub open spec fn NumSensorAxes(s: S, sensor_id: UInt32) -> int;
pub open spec fn ReadingsInAxisOrder(s: S, sensor_id: UInt32, readings: [SENSOR_READING; 1]) -> bool;
pub open spec fn NotificationInterval(s: S, sensor_id: UInt32) -> int;
pub open spec fn SensorUpdatePeriod(s: S, sensor_id: UInt32) -> int;
pub open spec fn ReadingsAreLatest(s: S, sensor_id: UInt32, readings: [SENSOR_READING; 1]) -> bool;
pub open spec fn SupportsContinuousUpdate(sensor: SensorId) -> bool;
pub open spec fn IsSensorUpdateImplemented(s: S) -> bool;

} // verus!
