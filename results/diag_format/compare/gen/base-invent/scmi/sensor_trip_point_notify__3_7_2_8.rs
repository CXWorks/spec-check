pub open spec fn sensor_trip_point_notify__3_7_2_8_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == T0 ==> (old_s.sensor_event_control == new_s.sensor_event_control))
    && (result == NOT_FOUND ==> (old_s.sensor_id == sensor_id))
    && (result == INVALID_PARAMETERS ==> (old_s.sensor_event_control != new_s.sensor_event_control))
    && (result == NOT_SUPPORTED ==> (old_s.sensor_id == sensor_id))
}