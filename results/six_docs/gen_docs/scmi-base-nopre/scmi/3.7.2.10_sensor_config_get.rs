pub open spec fn sensor_config_get_spec(result: int32, sensor_config: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> sensor_config == 0)
    && (result == SUCCESS ==> (sensor_config != 0))
    && (result != SUCCESS ==> (sensor_config == 0))
    && (result == SUCCESS ==> (new_s.sensor_config == old_s.sensor_config))
}