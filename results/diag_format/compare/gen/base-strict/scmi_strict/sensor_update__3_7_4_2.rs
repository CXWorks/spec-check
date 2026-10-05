pub open spec fn sensor_update__3_7_4_2_spec(agent_id: UInt32, sensor_id: UInt32, readings: SENSOR_READING, old_s: S, new_s: S) -> bool {
    (SensorEnabled(sensor_id) ==> true)
    && (ContinuousUpdateRequested(RecipientAgent(), sensor_id) ==> true)
    && (agent_id == 0)
    && (IsScalarSensor(sensor_id) ==> NumReadings(readings) == 1)
    && (IsAxisSensor(sensor_id) ==> NumReadings(readings) == NumSensorAxes(sensor_id))
    && (IsAxisSensor(sensor_id) ==> ReadingsInAxisOrder(sensor_id, readings))
    && (NotificationInterval(sensor_id) >= SensorUpdatePeriod(sensor_id))
    && (ReadingsAreLatest(sensor_id, readings))
    && ((exists|s: SensorId| SupportsContinuousUpdate(s)) ==> IsSensorUpdateImplemented())
}