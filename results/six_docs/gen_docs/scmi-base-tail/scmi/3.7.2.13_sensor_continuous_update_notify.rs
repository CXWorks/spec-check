pub open spec fn sensor_continuous_update_notify_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    || (result == SCMI_NOT_FOUND)
    || (result == SCMI_NOT_SUPPORTED)
    || (result == SCMI_INVALID_PARAMETERS)
}