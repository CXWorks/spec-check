pub open spec fn sensor_continuous_update_notify__3_7_2_13_spec(result: int32, old_s: S, new_s: S) -> bool {
    (!IsValidSensorId(sensor_id) ==> ResultEqual(result, NOT_FOUND))
    && (!SensorSupportsContinuousUpdateNotify(sensor_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidNotifyEnable(notify_enable) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> SensorUpdateNotifyEnabled(agent, sensor_id) == (notify_enable[0] == 1))
}