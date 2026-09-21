pub open spec fn 3.7.2.11_sensor_config_set_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (new_s.sensor_config == old_s.sensor_config))
    && (result == SCMI_NOT_FOUND ==> (old_s.sensor_id as int) < 0)
    && (result == SCMI_INVALID_PARAMETERS ==> (old_s.sensor_config as int) < 0)
    && (result == SCMI_NOT_SUPPORTED ==> (old_s.sensor_config as int) < 0)
}