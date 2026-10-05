pub open spec fn sensor_trip_point_event__3_7_4_1_spec(agent_id: u32, sensor_id: u32, trip_point_desc: u32, old_s: S, new_s: S) -> bool {
    (agent_id == 0)
    && SensorTripPointNotificationRequested(old_s, sensor_id, (trip_point_desc & 0xFF) as int)
}
