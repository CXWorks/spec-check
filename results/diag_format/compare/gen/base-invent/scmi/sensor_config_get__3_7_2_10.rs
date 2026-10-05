pub open spec fn sensor_config_get__3_7_2_10_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !SensorExists(old_s, sensor_id))
    && (result == SUCCESS ==> (SensorExists(old_s, sensor_id) && new_s == old_s))
}