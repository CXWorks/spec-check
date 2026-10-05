pub open spec fn sensor_config_set__3_7_2_11_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !SensorExists(old_s, sensor_id))
    && (result == INVALID_PARAMETERS ==> (sensor_update_interval == 0 || sensor_update_interval < 0 || (sensor_update_interval > 0 && (sensor_update_interval as int) < 0)))
    && (result == NOT_SUPPORTED ==> (sensor_id != 0 && (sensor_update_interval != 0 || timestamp_reporting != 0 || sensor_state != 0)))
    && (result == SUCCESS ==> (SensorExists(old_s, sensor_id) && SensorConfigSet(old_s, new_s, sensor_id, sensor_update_interval, timestamp_reporting, sensor_state)))
}