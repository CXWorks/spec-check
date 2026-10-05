pub open spec fn sensor_config_get__3_7_2_10_spec(result: Int32, sensor_config: UInt32, old_s: S, new_s: S) -> bool {
    (!SensorExists(sensor_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (
        ResultEqual(result, SUCCESS)
        && sensor_config[31:11] == SensorAt(old_s, sensor_id(old_s)).update_interval
        && (SensorSupportsUpdateInterval(old_s, sensor_id(old_s)) ==> sensor_config[31:11] != 0)
        && (SensorSupportsUpdateInterval(old_s, sensor_id(old_s)) ==> !(!SensorSupportsUpdateInterval(old_s, sensor_id(old_s))))
        && sensor_config[1] == (SensorAt(old_s, sensor_id(old_s)).timestamped ? 1 : 0)
        && sensor_config[0] == (SensorAt(old_s, sensor_id(old_s)).enabled ? 1 : 0)
    ))
    && (old_s == new_s)
}