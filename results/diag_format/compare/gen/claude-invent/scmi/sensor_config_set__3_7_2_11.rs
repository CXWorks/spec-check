pub open spec fn sensor_config_set__3_7_2_11_spec(sensor_id: u32, sensor_config: u32, status: i32, old_s: S, new_s: S) -> bool {
    (!SensorExists(old_s, sensor_id) ==> (status == NOT_FOUND && new_s == old_s))
    && ((SensorExists(old_s, sensor_id) && SensorConfigParamsInvalid(old_s, sensor_id, sensor_config)) ==> (status == INVALID_PARAMETERS && new_s == old_s))
    && ((SensorExists(old_s, sensor_id) && !SensorConfigParamsInvalid(old_s, sensor_id, sensor_config) && !SensorConfigSupported(old_s, sensor_id, sensor_config)) ==> (status == NOT_SUPPORTED && new_s == old_s))
    && ((SensorExists(old_s, sensor_id) && !SensorConfigParamsInvalid(old_s, sensor_id, sensor_config) && SensorConfigSupported(old_s, sensor_id, sensor_config)) ==> (
        status == SUCCESS
        && (SensorEnabled(new_s, sensor_id) == ((sensor_config & 1u32) == 1u32))
        && (SensorTimestampEnabled(new_s, sensor_id) == (((sensor_config >> 1u32) & 1u32) == 1u32))
        && ((((sensor_config >> 11u32) & 0x1F_FFFFu32) == 0u32) ==> SensorUpdateInterval(new_s, sensor_id) == SensorUpdateInterval(old_s, sensor_id))
        && ((((sensor_config >> 11u32) & 0x1F_FFFFu32) != 0u32) ==> SensorUpdateIntervalConfigured(
                old_s,
                new_s,
                sensor_id,
                (sensor_config >> 16u32) & 0xFFFFu32,
                (sensor_config >> 11u32) & 0x1Fu32,
                ((sensor_config >> 10u32) & 1u32) == 1u32,
                ((sensor_config >> 9u32) & 1u32) == 1u32))
        && OtherSensorsUnchanged(old_s, new_s, sensor_id)
    ))
}
