use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type SensorId = u32;

pub type AgentId = u32;

pub struct SENSOR_READING {
    pub num_readings: nat,
    pub timestamp: nat,
}

pub struct S {
    pub dummy: nat,
}

pub open spec fn SensorEnabled(sensor_id: UInt32) -> bool;

pub open spec fn RecipientAgent() -> AgentId;

pub open spec fn ContinuousUpdateRequested(agent: AgentId, sensor_id: UInt32) -> bool;

pub open spec fn IsScalarSensor(sensor_id: UInt32) -> bool;

pub open spec fn IsAxisSensor(sensor_id: UInt32) -> bool;

pub open spec fn NumReadings(readings: SENSOR_READING) -> nat;

pub open spec fn NumSensorAxes(sensor_id: UInt32) -> nat;

pub open spec fn ReadingsInAxisOrder(sensor_id: UInt32, readings: SENSOR_READING) -> bool;

pub open spec fn NotificationInterval(sensor_id: UInt32) -> nat;

pub open spec fn SensorUpdatePeriod(sensor_id: UInt32) -> nat;

pub open spec fn ReadingsAreLatest(sensor_id: UInt32, readings: SENSOR_READING) -> bool;

pub open spec fn SupportsContinuousUpdate(s: SensorId) -> bool;

pub open spec fn IsSensorUpdateImplemented() -> bool;

} // verus!
