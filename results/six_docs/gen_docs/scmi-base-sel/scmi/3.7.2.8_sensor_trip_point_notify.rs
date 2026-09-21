pub open spec fn sensor_trip_point_notify_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.sensor_event_control & 0x1 != 0 || old_s.sensor_id == 0))
    && (result == SCMI_NOT_FOUND ==> !old_s.sensor_exists(old_s.sensor_id))
    && (result == SCMI_NOT_SUPPORTED ==> !old_s.supports_trip_point_notifications(old_s.sensor_id))
    && (result == SCMI_SUCCESS ==> (old_s.sensor_event_control & 0x1 == 0 || old_s.sensor_event_control & 0x1 == 1))
    && (result == SCMI_SUCCESS ==> new_s.sensor_event_control == old_s.sensor_event_control)
    && (result == SCMI_SUCCESS ==> new_s.sensor_id == old_s.sensor_id)
    && (result == SCMI_SUCCESS ==> new_s.supports_trip_point_notifications(new_s.sensor_id) == old_s.supports_trip_point_notifications(old_s.sensor_id))
    && (result == SCMI_SUCCESS ==> new_s.sensor_exists(new_s.sensor_id) == old_s.sensor_exists(old_s.sensor_id))
}