pub open spec fn sensor_trip_point_notify__3_7_2_8_spec(status: Int32, sensor_id: UInt32, sensor_event_control: UInt32, old_s: S, new_s: S) -> bool {
    (!IsExistingSensor(sensor_id) ==> ResultEqual(status, NOT_FOUND))
    && (!IsValidSensorEventControl(sensor_event_control) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (!SensorSupportsTripPointNotify(sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
    && (ResultEqual(status, SUCCESS) ==> SensorAt(new_s, sensor_id).trip_point_notify_enabled == (sensor_event_control as int & 0x1))
}