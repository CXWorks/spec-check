pub open spec fn sensor_config_get__3_7_2_10_spec(result: Int32, sensor_config: UInt32, old_s: S, new_s: S) -> bool {
    (!SensorExists(old_s, sensor_id) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (
        (SensorUpdateIntervalSupported(old_s, sensor_id) ==> (
            Bits(sensor_config, 31, 16) == SensorUpdateIntervalSec(old_s, sensor_id)
            && Bits(sensor_config, 15, 11) == SensorUpdateIntervalExponent(old_s, sensor_id)
        ))
        && (!SensorUpdateIntervalSupported(old_s, sensor_id) ==> Bits(sensor_config, 31, 11) == 0)
        && (SensorIsTimestamped(old_s, sensor_id) ==> Bits(sensor_config, 1, 1) == 1)
        && (!SensorIsTimestamped(old_s, sensor_id) ==> Bits(sensor_config, 1, 1) == 0)
        && (SensorIsEnabled(old_s, sensor_id) ==> Bits(sensor_config, 0, 0) == 1)
        && (!SensorIsEnabled(old_s, sensor_id) ==> Bits(sensor_config, 0, 0) == 0)
    ))
    && (old_s == new_s)
}