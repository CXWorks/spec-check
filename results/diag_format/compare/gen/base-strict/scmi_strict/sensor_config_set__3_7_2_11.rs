pub open spec fn sensor_config_set__3_7_2_11_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!SensorExists(old_s, sensor_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidSensorConfig(old_s, sensor_id, sensor_config) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!SensorSupportsConfig(old_s, sensor_id, sensor_config) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> (SensorAt(new_s, sensor_id).enabled == (Bits(sensor_config, 0, 0) == 1)))
    && (ResultEqual(result, SUCCESS) ==> (SensorAt(new_s, sensor_id).timestamp_enabled == (Bits(sensor_config, 1, 1) == 1)))
    && (ResultEqual(result, SUCCESS) ==> (Bits(sensor_config, 31, 11) == 0 ==> UpdateIntervalUnchanged(old_s, new_s, sensor_id)))
    && (ResultEqual(result, SUCCESS) ==> ((Bits(sensor_config, 31, 11) != 0 && Bits(sensor_config, 10, 10) == 1) ==> SensorAt(new_s, sensor_id).update_interval == ClosestSupportedInterval(old_s, sensor_id, RequestedInterval(Bits(sensor_config, 31, 16), Bits(sensor_config, 15, 11)))))
    && (ResultEqual(result, SUCCESS) ==> ((Bits(sensor_config, 31, 11) != 0 && Bits(sensor_config, 10, 10) == 0 && Bits(sensor_config, 9, 9) == 1) ==> SensorAt(new_s, sensor_id).update_interval == RoundUpSupportedInterval(old_s, sensor_id, RequestedInterval(Bits(sensor_config, 31, 16), Bits(sensor_config, 15, 11)))))
    && (ResultEqual(result, SUCCESS) ==> ((Bits(sensor_config, 31, 11) != 0 && Bits(sensor_config, 10, 10) == 0 && Bits(sensor_config, 9, 9) == 0) ==> SensorAt(new_s, sensor_id).update_interval == RoundDownSupportedInterval(old_s, sensor_id, RequestedInterval(Bits(sensor_config, 31, 16), Bits(sensor_config, 15, 11)))))
}