pub open spec fn sensor_continuous_update_notify__3_7_2_13_spec(sensor_id: UInt32, notify_enable: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidSensor(old_s, sensor_id) ==> ResultEqual(status, NOT_FOUND))
  && (!SensorSupportsContinuousUpdateNotify(old_s, sensor_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!IsSupportedNotifyEnable(old_s, notify_enable) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (result ==> ResultEqual(status, SUCCESS))
  && (result ==> SensorUpdateNotifyEnabled(new_s, sensor_id, agent) == (Bits(notify_enable, 0, 0) == 1))
  && ((IsValidSensor(old_s, sensor_id) &&
       SensorSupportsContinuousUpdateNotify(old_s, sensor_id) &&
       IsSupportedNotifyEnable(old_s, notify_enable))
    ==> ResultEqual(status, SUCCESS))
}