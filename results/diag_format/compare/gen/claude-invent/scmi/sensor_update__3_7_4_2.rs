pub open spec fn sensor_update__3_7_4_2_spec(agent_id: UInt32, sensor_id: UInt32, readings_count: UInt32, old_s: S, new_s: S) -> bool {
    (agent_id == 0)
    && IsSensorEnabled(old_s, sensor_id)
    && IsSensorContinuousUpdateNotifyRequested(old_s, sensor_id)
    && (SensorReportsAxes(old_s, sensor_id) ==> (readings_count as int) == (SensorNumAxes(old_s, sensor_id) as int))
    && (!SensorReportsAxes(old_s, sensor_id) ==> readings_count == 1)
}
