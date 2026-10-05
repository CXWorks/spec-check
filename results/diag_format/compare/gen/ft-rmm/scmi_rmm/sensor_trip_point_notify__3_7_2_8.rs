pub open spec fn sensor_trip_point_notify__3_7_2_8_spec(sensor_id: UInt32, sensor_event_control: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsExistingSensor(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidSensorEventControl(old_s, sensor_event_control) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!SensorSupportsTripPointNotify(old_s, sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> SensorAt(new_s, sensor_id).trip_point_notify_enabled == sensor_event_control[0])
  && ((!(IsExistingSensor(old_s, sensor_id)) &&
       IsValidSensorEventControl(old_s, sensor_event_control) &&
       SensorSupportsTripPointNotify(old_s, sensor_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> SensorAt(new_s, sensor_id).trip_point_notify_enabled == SensorAt(old_s, sensor_id).trip_point_notify_enabled)
}