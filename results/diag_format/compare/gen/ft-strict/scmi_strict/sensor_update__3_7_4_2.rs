pub open spec fn sensor_update__3_7_4_2_spec(agent_id: UInt32, sensor_id: UInt32, readings: [SENSOR_READING; 1], old_s: S, new_s: S) -> bool {
  (SensorEnabled(new_s, sensor_id))
  && (ContinuousUpdateRequested(new_s, RecipientAgent(), sensor_id))
  && (agent_id == 0)
  && (IsScalarSensor(new_s, sensor_id) ==> NumReadings(new_s, readings) == 1)
  && (IsAxisSensor(new_s, sensor_id) ==> NumReadings(new_s, readings) == NumSensorAxes(new_s, sensor_id))
  && (IsAxisSensor(new_s, sensor_id) ==> ReadingsInAxisOrder(new_s, sensor_id, readings))
  && (NotificationInterval(new_s, sensor_id) >= SensorUpdatePeriod(new_s, sensor_id))
  && (ReadingsAreLatest(new_s, sensor_id, readings))
  && ((exists s: SensorId| SupportsContinuousUpdate(s)) ==> IsSensorUpdateImplemented(new_s))
  && ((!(SensorEnabled(old_s, sensor_id)) &&
       !(ContinuousUpdateRequested(old_s, RecipientAgent(), sensor_id)) &&
       agent_id != 0 &&
       !(IsScalarSensor(old_s, sensor_id) ==> NumReadings(old_s, readings) == 1) &&
       !(IsAxisSensor(old_s, sensor_id) ==> NumReadings(old_s, readings) == NumSensorAxes(old_s, sensor_id)) &&
       !(IsAxisSensor(old_s, sensor_id) ==> ReadingsInAxisOrder(old_s, sensor_id, readings)) &&
       !(NotificationInterval(old_s, sensor_id) >= SensorUpdatePeriod(old_s, sensor_id)) &&
       !(ReadingsAreLatest(old_s, sensor_id, readings)) &&
       !((exists s: SensorId| SupportsContinuousUpdate(s)) ==> IsSensorUpdateImplemented(old_s)))
    ==> (SensorEnabled(new_s, sensor_id) &&
        ContinuousUpdateRequested(new_s, RecipientAgent(), sensor_id) &&
        agent_id == 0 &&
        (IsScalarSensor(new_s, sensor_id) ==> NumReadings(new_s, readings) == 1) &&
        (IsAxisSensor(new_s, sensor_id) ==> NumReadings(new_s, readings) == NumSensorAxes(new_s, sensor_id)) &&
        (IsAxisSensor(new_s, sensor_id) ==> ReadingsInAxisOrder(new_s, sensor_id, readings)) &&
        (NotificationInterval(new_s, sensor_id) >= SensorUpdatePeriod(new_s, sensor_id)) &&
        (ReadingsAreLatest(new_s, sensor_id, readings)) &&
        ((exists s: SensorId| SupportsContinuousUpdate(s)) ==> IsSensorUpdateImplemented(new_s))))
}