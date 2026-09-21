pub open spec fn 3.7.2.13_sensor_continuous_update_notify_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (result == SCMI_NOT_FOUND ==> (old_s.sensor_id as int) < 0)
    && (result == SCMI_NOT_SUPPORTED ==> (old_s.sensor_id as int) >= 0)
    && (result == SCMI_INVALID_PARAMETERS ==> (old_s.notify_enable as int) != 0)
}