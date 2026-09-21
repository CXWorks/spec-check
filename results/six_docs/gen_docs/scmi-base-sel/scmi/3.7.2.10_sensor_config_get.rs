pub open spec fn 3.7.2.10_sensor_config_get_spec(result: int32, sensor_config: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (sensor_config == old_s.sensor_config))
    && (result == SCMI_NOT_FOUND ==> (sensor_config == 0))
    && (result != SCMI_SUCCESS ==> (sensor_config == 0))
}