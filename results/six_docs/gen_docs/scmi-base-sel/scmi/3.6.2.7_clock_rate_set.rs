pub open spec fn clock_rate_set_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (result == SCMI_NOT_FOUND)
    && (result == SCMI_INVALID_PARAMETERS)
    && (result == SCMI_BUSY)
    && (result == SCMI_DENIED)
}