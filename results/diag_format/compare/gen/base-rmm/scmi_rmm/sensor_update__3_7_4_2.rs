pub open spec fn sensor_update__3_7_4_2_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==>
        AgentRequestedContinuousUpdates(old_s, new_s.agent_id, new_s.sensor_id)
        && SensorIsEnabled(old_s, new_s.sensor_id)
        && new_s.agent_id == 0
        && (IsScalarSensor(old_s, new_s.sensor_id) ==> Len(new_s.readings) == 1)
        && (IsAxisSensor(old_s, new_s.sensor_id) ==> Len(new_s.readings) == NumSensorAxes(old_s, new_s.sensor_id))
        && (IsAxisSensor(old_s, new_s.sensor_id) ==> AllAxesReportedInOrder(new_s.readings))
        && new_s.readings == LatestSensorValues(old_s, new_s.sensor_id)
        && NotificationInterval(old_s, new_s.sensor_id) >= ConfiguredUpdatePeriod(old_s, new_s.sensor_id))
    && (result != RSI_SUCCESS ==> true)
}