pub open spec fn sensor_config_set__3_7_2_11_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsExistingSensor(sensor_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!AreValidSensorConfigParameters(sensor_config(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsSensorConfigSupported(sensor_id(old_s), sensor_config(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> (SensorAt(new_s, sensor_id(old_s)).enabled == sensor_config(old_s)[0]
        && SensorAt(new_s, sensor_id(old_s)).timestamp_enabled == sensor_config(old_s)[1]
        && (sensor_config(old_s)[31:11] != 0 ==> SensorAt(new_s, sensor_id(old_s)).update_interval == RoundedUpdateInterval(sensor_config(old_s)[31:16], sensor_config(old_s)[15:11], sensor_config(old_s)[15:11], sensor_config(old_s)[10], sensor_config(old_s)[9])))
    )
}