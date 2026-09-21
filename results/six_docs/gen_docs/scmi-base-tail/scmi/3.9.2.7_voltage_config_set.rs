pub open spec fn voltage_config_set_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> true)
    && (result == SCMI_NOT_FOUND ==> true)
    && (result == SCMI_INVALID_PARAMETERS ==> true)
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_DENIED ==> true)
}