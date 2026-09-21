pub open spec fn sensor_trip_point_notify_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (old_s.sensor_exists(old_s.sensor_id) && old_s.sensor_event_control(old_s.sensor_id) == new_s.sensor_event_control(old_s.sensor_id)))
    && (result == -1 ==> (old_s.sensor_id == 0 || !old_s.sensor_exists(old_s.sensor_id)))
    && (result == -2 ==> (old_s.sensor_event_control(old_s.sensor_id) & 0x1) != 0)
    && (result == -3 ==> !old_s.sensor_supports_trip_point_notifications(old_s.sensor_id))
    && (result != 0 && result != -1 && result != -2 && result != -3 ==> true)
}