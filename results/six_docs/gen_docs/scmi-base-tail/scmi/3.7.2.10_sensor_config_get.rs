pub open spec fn sensor_config_get_spec(result: int32, sensor_config: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_NOT_FOUND ==> sensor_config == 0)
    && (result == SCMI_SUCCESS ==> true)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND ==> true)
}