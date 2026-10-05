pub open spec fn sensor_update__3_7_4_2_spec(agent_id: uint32, sensor_id: uint32, readings: [SENSOR_READING; 1], old_s: S, new_s: S) -> bool {
  (AgentRequestedContinuousUpdates(new_s, agent_id, sensor_id))
  && (SensorIsEnabled(new_s, sensor_id))
  && (agent_id == 0)
  && (IsScalarSensor(new_s, sensor_id) ==> Len(new_s, readings) == 1)
  && (IsAxisSensor(new_s, sensor_id) ==> Len(new_s, readings) == NumSensorAxes(new_s, sensor_id))
  && (IsAxisSensor(new_s, sensor_id) ==> AllAxesReportedInOrder(new_s, readings))
  && (readings == LatestSensorValues(new_s, sensor_id))
  && (NotificationInterval(new_s, sensor_id) >= ConfiguredUpdatePeriod(new_s, sensor_id))
  && ((!(AgentRequestedContinuousUpdates(old_s, agent_id, sensor_id)) &&
       !(SensorIsEnabled(old_s, sensor_id)) &&
       agent_id != 0 &&
       !(IsScalarSensor(old_s, sensor_id)) &&
       !(IsAxisSensor(old_s, sensor_id)) &&
       !(IsAxisSensor(old_s, sensor_id)) &&
       !(readings == LatestSensorValues(old_s, sensor_id)) &&
       !(NotificationInterval(old_s, sensor_id) >= ConfiguredUpdatePeriod(old_s, sensor_id)))
    ==> (AgentRequestedContinuousUpdates(new_s, agent_id, sensor_id) &&
        SensorIsEnabled(new_s, sensor_id) &&
        agent_id == 0 &&
        Len(new_s, readings) == 1 &&
        Len(new_s, readings) == NumSensorAxes(new_s, sensor_id) &&
        AllAxesReportedInOrder(new_s, readings) &&
        readings == LatestSensorValues(new_s, sensor_id) &&
        NotificationInterval(new_s, sensor_id) >= ConfiguredUpdatePeriod(new_s, sensor_id)))
}