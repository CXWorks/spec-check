pub open spec fn sensor_continuous_update_notify__3_7_2_13_spec(sensor_id: uint32, notify_enable: [uint32; 2], status: int32, old_s: S, new_s: S) -> bool {
  (!IsValidSensorId(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (!SensorSupportsContinuousUpdateNotify(old_s, sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!IsValidNotifyEnable(old_s, notify_enable[0]) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> SensorUpdateNotifyEnabled(new_s, 0, sensor_id) == (notify_enable[0] == 1))
  && ((IsValidSensorId(old_s, sensor_id) &&
       SensorSupportsContinuousUpdateNotify(old_s, sensor_id) &&
       IsValidNotifyEnable(old_s, notify_enable[0]))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> SensorUpdateNotifyEnabled(new_s, 0, sensor_id) == SensorUpdateNotifyEnabled(old_s, 0, sensor_id))
}