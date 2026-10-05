pub open spec fn sensor_trip_point_notify__3_7_2_8_spec(status: Int32, sensor_id: UInt32, sensor_event_control: UInt32, old_s: S, new_s: S) -> bool {
    (!SensorExists(sensor_id) ==> ResultEqual(status, NOT_FOUND))
    && (!IsValidSensorEventControl(sensor_event_control) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (!SupportsTripPointEventNotifications(sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
    && (ResultEqual(status, SUCCESS) ==> (Bits(sensor_event_control, 0, 0) == 1 ==> TripPointNotificationsEnabled(sensor_id)))
    && (ResultEqual(status, SUCCESS) ==> (Bits(sensor_event_control, 0, 0) == 0 ==> !TripPointNotificationsEnabled(sensor_id)))
}