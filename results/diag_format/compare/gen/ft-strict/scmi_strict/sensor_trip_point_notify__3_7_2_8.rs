pub open spec fn sensor_trip_point_notify__3_7_2_8_spec(sensor_id: UInt32, sensor_event_control: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!SensorExists(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidSensorEventControl(old_s, sensor_event_control) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!SupportsTripPointEventNotifications(old_s, sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) && Bits(sensor_event_control, 0, 0) == 1 ==> TripPointNotificationsEnabled(new_s, sensor_id))
  && (ResultEqual(status, SUCCESS) && Bits(sensor_event_control, 0, 0) == 0 ==> !TripPointNotificationsEnabled(new_s, sensor_id))
  && ((SensorExists(old_s, sensor_id) &&
       IsValidSensorEventControl(old_s, sensor_event_control) &&
       SupportsTripPointEventNotifications(old_s, sensor_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> !TripPointNotificationsEnabled(new_s, sensor_id))
}