pub open spec fn reset_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (result == SCMI_NOT_FOUND)
    && (result == SCMI_INVALID_PARAMETERS)
    && (result == SCMI_GENERIC_ERROR)
    && (result == SCMI_DENIED)
}