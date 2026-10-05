pub open spec fn sensor_continuous_update_notify__3_7_2_13_spec(status: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidSensor(old_s, sensor_id(old_s)) ==> ResultEqual(status, NOT_FOUND))
    && (!SensorSupportsContinuousUpdateNotify(old_s, sensor_id(old_s)) ==> ResultEqual(status, NOT_SUPPORTED))
    && (!IsSupportedNotifyEnable(old_s, Bits(notify_enable(old_s), 0, 0)) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (ResultEqual(status, SUCCESS) ==> SensorUpdateNotifyEnabled(new_s, sensor_id(old_s), agent(old_s)) == (Bits(notify_enable(old_s), 0, 0) == 1))
}